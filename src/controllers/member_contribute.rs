#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::controllers::trust_score_utils::{
    update_trust_score_for_contribution, update_trust_score_for_missed_payment,
};
use crate::models::_entities::{contributions, groups, members};
use chrono::Utc;
use loco_rs::prelude::*;
use num_traits::cast::FromPrimitive;
use reqwest::Client;
use sea_orm::{ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ContributeParams {
    pub group_id: i32,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct ContributeResponse {
    pub success: bool,
    pub contribution_id: i32,
    pub amount: f64,
    pub message: String,
    pub moolre_status: Option<String>,
}

#[debug_handler]
pub async fn contribute(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<ContributeParams>,
) -> Result<Json<ContributeResponse>> {
    // Validate amount
    if params.amount <= 0.0 {
        return Ok(Json(ContributeResponse {
            success: false,
            contribution_id: 0,
            amount: params.amount,
            message: "Amount must be greater than 0".to_string(),
            moolre_status: None,
        }));
    }

    let member_id: i32 = auth
        .claims
        .pid
        .parse()
        .map_err(|_| Error::Unauthorized("Invalid member ID".to_string()))?;

    let member = members::Entity::find_by_id(member_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    if member.group_id != Some(params.group_id) {
        return Ok(Json(ContributeResponse {
            success: false,
            contribution_id: 0,
            amount: params.amount,
            message: "You are not a member of this group".to_string(),
            moolre_status: None,
        }));
    }

    let group = groups::Entity::find_by_id(params.group_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let group_name = group.name.clone();

    let external_ref = format!(
        "CONTRIB_{}_{}_{}",
        member_id,
        params.group_id,
        Utc::now().timestamp()
    );

    let amount_decimal = Decimal::from_f64(params.amount).unwrap_or(Decimal::ZERO);

    let new_contribution = contributions::ActiveModel {
        member_id: Set(member_id),
        group_id: Set(params.group_id),
        amount: Set(amount_decimal),
        status: Set("pending".to_string()),
        moolre_ref: Set(Some(external_ref.clone())),
        date: Set(Some(Utc::now().naive_utc())),
        ..Default::default()
    };

    let contribution = new_contribution.insert(&ctx.db).await?;
    let contribution_id = contribution.id;

    // Moolre Payment API - uses X-API-PUBKEY
    let moolre_user = std::env::var("MOOLRE_API_USER").unwrap_or_default();
    let moolre_pubkey = std::env::var("MOOLRE_API_PUBKEY").unwrap_or_default();
    let account_number =
        std::env::var("MOOLRE_ACCOUNT_NUMBER").unwrap_or_else(|_| "10878506070937".to_string());
    let base_url = std::env::var("MOOLRE_BASE_URL")
        .unwrap_or_else(|_| "https://sandbox.moolre.com".to_string());

    let client = Client::new();
    let payload = serde_json::json!({
        "type": 1,
        "channel": "13",
        "currency": "GHS",
        "payer": member.phone,
        "amount": params.amount.to_string(),
        "externalref": external_ref,
        "reference": format!("Contribution to {}", group_name),
        "accountnumber": account_number
    });

    let moolre_response = client
        .post(format!("{}/open/transact/payment", base_url))
        .header("X-API-USER", &moolre_user)
        .header("X-API-PUBKEY", &moolre_pubkey)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    let moolre_status = match moolre_response {
        Ok(resp) => {
            let resp_body: serde_json::Value = resp.json().await.unwrap_or_default();
            let status = resp_body
                .get("status")
                .and_then(|s| s.as_str())
                .unwrap_or("unknown");

            if status == "1" {
                let mut contrib_active: contributions::ActiveModel = contribution.into();
                contrib_active.status = Set("completed".to_string());
                contrib_active.update(&ctx.db).await?;

                // Update member's total contributed
                let mut member_active: members::ActiveModel = member.clone().into();
                let current_member_total = member_active
                    .total_contributed
                    .take()
                    .unwrap_or(Some(Decimal::ZERO))
                    .unwrap_or(Decimal::ZERO);
                member_active.total_contributed = Set(Some(current_member_total + amount_decimal));

                // Update on_time_payments
                let current_on_time = member_active
                    .on_time_payments
                    .take()
                    .unwrap_or(Some(0))
                    .unwrap_or(0);
                member_active.on_time_payments = Set(Some(current_on_time + 1));

                // Update total_payments
                let current_total_payments = member_active
                    .total_payments
                    .take()
                    .unwrap_or(Some(0))
                    .unwrap_or(0);
                member_active.total_payments = Set(Some(current_total_payments + 1));

                member_active.update(&ctx.db).await?;

                // Update group's total contributed
                let mut group_active: groups::ActiveModel = group.clone().into();
                let current_total = group_active
                    .total_contributed
                    .take()
                    .unwrap_or(Some(Decimal::ZERO))
                    .unwrap_or(Decimal::ZERO);
                let new_total = current_total + amount_decimal;
                group_active.total_contributed = Set(Some(new_total));
                group_active.update(&ctx.db).await?;

                // Update trust score
                let was_on_time = true; // Contributions are always on-time if completed
                match update_trust_score_for_contribution(&ctx.db, member_id, was_on_time).await {
                    Ok(_) => tracing::info!("Trust score updated for member {}", member_id),
                    Err(e) => tracing::error!("Failed to update trust score: {}", e),
                }

                Some("success".to_string())
            } else if status == "0" {
                // Payment pending - don't update anything yet
                Some(format!("pending: {:?}", resp_body))
            } else {
                // Payment failed - increment total_payments but not on_time
                let mut member_active: members::ActiveModel = member.clone().into();
                let current_total_payments = member_active
                    .total_payments
                    .take()
                    .unwrap_or(Some(0))
                    .unwrap_or(0);
                member_active.total_payments = Set(Some(current_total_payments + 1));
                member_active.update(&ctx.db).await?;

                // Update trust score for missed payment
                match update_trust_score_for_missed_payment(&ctx.db, member_id).await {
                    Ok(_) => {
                        tracing::info!("Trust score updated for missed payment: {}", member_id)
                    }
                    Err(e) => tracing::error!("Failed to update trust score: {}", e),
                }

                Some(format!("failed: {:?}", resp_body))
            }
        }
        Err(e) => Some(format!("error: {}", e)),
    };

    Ok(Json(ContributeResponse {
        success: moolre_status == Some("success".to_string()),
        contribution_id,
        amount: params.amount,
        message: if moolre_status == Some("success".to_string()) {
            format!(
                "Successfully contributed GHS {} to {}",
                params.amount, group_name
            )
        } else {
            format!("Contribution initiated but pending confirmation")
        },
        moolre_status,
    }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/member/contribute")
        .add("/", post(contribute))
}

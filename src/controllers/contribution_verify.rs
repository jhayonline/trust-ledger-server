#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::models::_entities::{contributions, groups, members};
use loco_rs::prelude::*;
use sea_orm::{ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct VerifyContributionParams {
    pub contribution_id: i32,
    pub otp_code: String,
}

#[derive(Serialize)]
pub struct VerifyContributionResponse {
    pub success: bool,
    pub message: String,
    pub status: String,
}

#[debug_handler]
pub async fn verify(
    State(ctx): State<AppContext>,
    Json(params): Json<VerifyContributionParams>,
) -> Result<Json<VerifyContributionResponse>> {
    // Find the contribution
    let contribution = contributions::Entity::find_by_id(params.contribution_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Check if already completed
    if contribution.status == "completed" {
        return Ok(Json(VerifyContributionResponse {
            success: false,
            message: "Contribution already verified".to_string(),
            status: "completed".to_string(),
        }));
    }

    // Check if contribution has a Moolre reference
    let moolre_ref = match contribution.moolre_ref {
        Some(ref r) if !r.is_empty() => r.clone(),
        _ => {
            return Ok(Json(VerifyContributionResponse {
                success: false,
                message: "Contribution has no Moolre reference. Please try again.".to_string(),
                status: "pending".to_string(),
            }));
        }
    };

    // Verify OTP with Moolre
    let moolre_user = std::env::var("MOOLRE_API_USER").unwrap_or_default();
    let moolre_key = std::env::var("MOOLRE_API_KEY").unwrap_or_default();
    let base_url =
        std::env::var("MOOLRE_BASE_URL").unwrap_or_else(|_| "https://api.moolre.com".to_string());

    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "type": 1,
        "idtype": 2,
        "id": moolre_ref,
        "accountnumber": "10878506070937"
    });

    let response = client
        .post(format!("{}/open/transact/status", base_url))
        .header("X-API-USER", &moolre_user)
        .header("X-API-KEY", &moolre_key)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(resp) => {
            let resp_body: serde_json::Value = resp.json().await.unwrap_or_default();
            let txstatus = resp_body
                .get("data")
                .and_then(|d| d.get("txstatus"))
                .and_then(|s| s.as_i64())
                .unwrap_or(0);

            if txstatus == 1 {
                // Update contribution status
                let mut contrib_active: contributions::ActiveModel = contribution.clone().into();
                contrib_active.status = Set("completed".to_string());
                let updated_contrib = contrib_active.update(&ctx.db).await?;

                // Update member's total_contributed
                let member = members::Entity::find_by_id(updated_contrib.member_id)
                    .one(&ctx.db)
                    .await?;
                if let Some(m) = member {
                    let mut member_active: members::ActiveModel = m.into();
                    let current = member_active
                        .total_contributed
                        .take()
                        .unwrap_or(Some(Decimal::ZERO))
                        .unwrap_or(Decimal::ZERO);
                    member_active.total_contributed = Set(Some(current + updated_contrib.amount));
                    member_active.update(&ctx.db).await?;
                }

                // Update group total
                let group = groups::Entity::find_by_id(updated_contrib.group_id)
                    .one(&ctx.db)
                    .await?;
                if let Some(g) = group {
                    let mut group_active: groups::ActiveModel = g.into();
                    let current = group_active
                        .total_contributed
                        .take()
                        .unwrap_or(Some(Decimal::ZERO))
                        .unwrap_or(Decimal::ZERO);
                    group_active.total_contributed = Set(Some(current + updated_contrib.amount));
                    group_active.update(&ctx.db).await?;
                }

                Ok(Json(VerifyContributionResponse {
                    success: true,
                    message: "Contribution verified successfully".to_string(),
                    status: "completed".to_string(),
                }))
            } else if txstatus == 0 {
                Ok(Json(VerifyContributionResponse {
                    success: false,
                    message: "Payment is still pending. Please wait.".to_string(),
                    status: "pending".to_string(),
                }))
            } else {
                Ok(Json(VerifyContributionResponse {
                    success: false,
                    message: "Payment failed. Please try again.".to_string(),
                    status: "failed".to_string(),
                }))
            }
        }
        Err(e) => {
            tracing::error!("Failed to check payment status: {}", e);
            Ok(Json(VerifyContributionResponse {
                success: false,
                message: "Failed to verify payment".to_string(),
                status: "pending".to_string(),
            }))
        }
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/contributions")
        .add("/verify", post(verify))
}

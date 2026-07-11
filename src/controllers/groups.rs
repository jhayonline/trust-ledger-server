#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::models::_entities::{groups, members};
use loco_rs::prelude::*;
use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateGroupParams {
    pub name: String,
    pub code: String,
    pub description: String,
    pub contribution_amount: Decimal,
    pub frequency: String,
    pub next_payout_date: String,
}

#[derive(Debug, Deserialize)]
pub struct AddMemberParams {
    pub name: string,
    pub phone: string,
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<groups::Model>>> {
    let all_groups = groups::Entity::find()
        .order_by_asc(groups::Column::Id)
        .all(&ctx.db)
        .await?;

    Ok(Json(all_groups))
}

#[debug_handler]
pub async fn show(
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Json<groups::Model>> {
    let group = groups::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    Ok(Json(group))
}

#[debug_handler]
pub async fn members_list(
    State(ctx): State<AppContext>,
    Path(group_id): Path<i32>,
) -> Result<Json<Vec<members::Model>>> {
    let members_list = members::Entity::find()
        .filter(members::Column::GroupId.eq(group_id))
        .all(&ctx.db)
        .await?;

    Ok(Json(members_list))
}

#[debug_handler]
pub async fn delete_group(
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>> {
    let group = groups::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Soft delete by setting is_active to false
    let mut group_active: groups::ActiveModel = group.into();
    group_active.is_active = Set(Some(false));
    group_active.update(&ctx.db).await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Group deleted successfully"
    })))
}

#[debug_handler]
pub async fn delete_member(
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>> {
    let member = members::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Store group_id before moving member
    let group_id = member.group_id;

    // Delete the member
    let member_active: members::ActiveModel = member.into();
    member_active.delete(&ctx.db).await?;

    // Update group member count
    if let Some(g_id) = group_id {
        let group = groups::Entity::find_by_id(g_id).one(&ctx.db).await?;
        if let Some(g) = group {
            let mut group_active: groups::ActiveModel = g.into();
            let current_count = group_active.member_count.take().flatten().unwrap_or(0);
            group_active.member_count = Set(Some(current_count - 1));
            group_active.update(&ctx.db).await?;
        }
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Member removed successfully"
    })))
}

#[debug_handler]
pub async fn create_group(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateGroupParams>,
) -> Result<Json<groups::Model>> {
    // Parse date - if empty, set to 30 days from now
    let next_date = if params.next_payout_date.is_empty() {
        chrono::Utc::now().naive_utc() + chrono::Duration::days(30)
    } else {
        // Parse date in YYYY-MM-DD format
        let parsed = chrono::NaiveDate::parse_from_str(&params.next_payout_date, "%Y-%m-%d")
            .map_err(|_| Error::BadRequest("Invalid date format. Use YYYY-MM-DD".to_string()))?;
        parsed
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| Error::BadRequest("Invalid date".to_string()))?
    };

    let new_group = groups::ActiveModel {
        name: Set(params.name),
        code: Set(params.code),
        description: Set(Some(params.description)),
        contribution_amount: Set(Some(params.contribution_amount)),
        frequency: Set(params.frequency),
        member_count: Set(Some(0)),
        total_contributed: Set(Some(Decimal::ZERO)),
        next_payout_date: Set(next_date),
        is_active: Set(Some(true)),
        created_at: Set(Some(chrono::Utc::now().naive_utc())),
        ..Default::default()
    };

    let group = new_group.insert(&ctx.db).await?;
    Ok(Json(group))
}

#[debug_handler]
pub async fn add_member(
    State(ctx): State<AppContext>,
    Path(group_id): Path<i32>,
    Json(params): Json<AddMemberParams>,
) -> Result<Json<members::Model>> {
    // Check if group exists
    let group = groups::Entity::find_by_id(group_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Check if member with this phone already exists in this group
    let existing = members::Entity::find()
        .filter(members::Column::Phone.eq(&params.phone))
        .filter(members::Column::GroupId.eq(group_id))
        .one(&ctx.db)
        .await?;

    if let Some(_) = existing {
        return Err(Error::BadRequest(
            "Member with this phone already exists in the group".to_string(),
        ));
    }

    // Create new member
    let new_member = members::ActiveModel {
        name: Set(params.name),
        phone: Set(params.phone),
        trust_score: Set(Some(100)),
        group_id: Set(Some(group_id)),
        total_contributed: Set(Some(Decimal::ZERO)),
        on_time_payments: Set(Some(0)),
        total_payments: Set(Some(0)),
        last_missed_payment: Set(None),
        status: Set(Some("active".to_string())),
        joined_at: Set(Some(chrono::Utc::now().naive_utc())),
        ..Default::default()
    };

    let member = new_member.insert(&ctx.db).await?;

    // Update group member count
    let mut group_active: groups::ActiveModel = group.into();
    let current_count = group_active.member_count.take().flatten().unwrap_or(0);
    group_active.member_count = Set(Some(current_count + 1));
    group_active.update(&ctx.db).await?;

    Ok(Json(member))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/groups")
        .add("/", get(list))
        .add("/", post(create_group))
        .add("/{id}", get(show))
        .add("/{id}", delete(delete_group))
        .add("/{group_id}/members", get(members_list))
        .add("/{group_id}/members", post(add_member))
        .add("/members/{id}", delete(delete_member))
}

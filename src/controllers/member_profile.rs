#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::models::_entities::members;
use loco_rs::prelude::*;
use sea_orm::{ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UpdateProfileParams {
    pub name: String,
}

#[derive(Serialize)]
pub struct UpdateProfileResponse {
    pub success: bool,
    pub message: String,
    pub name: String,
}

#[debug_handler]
pub async fn update(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateProfileParams>,
) -> Result<Json<UpdateProfileResponse>> {
    // Get member ID from JWT claims
    let member_id: i32 = auth
        .claims
        .pid
        .parse()
        .map_err(|_| Error::Unauthorized("Invalid member ID".to_string()))?;

    // Find the member
    let member = members::Entity::find_by_id(member_id)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Update the name
    let mut member_active: members::ActiveModel = member.into();
    member_active.name = Set(params.name.clone());
    member_active.update(&ctx.db).await?;

    Ok(Json(UpdateProfileResponse {
        success: true,
        message: "Profile updated successfully".to_string(),
        name: params.name,
    }))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/member/profile")
        .add("/", put(update))
}

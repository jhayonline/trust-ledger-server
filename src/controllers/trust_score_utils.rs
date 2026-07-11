use crate::models::_entities::{contributions, groups, members, trust_score_history};
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};

/// Calculate trust score for a member based on their contribution history
pub async fn calculate_trust_score(
    db: &sea_orm::DatabaseConnection,
    member_id: i32,
) -> Result<i32> {
    // Get member details
    let member = members::Entity::find_by_id(member_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Get member's group
    let group_id = match member.group_id {
        Some(id) => id,
        None => return Ok(100), // No group, default score
    };

    let group = groups::Entity::find_by_id(group_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    // Calculate expected payments based on join date
    let join_date = member.joined_at.unwrap_or_else(|| Utc::now().naive_utc());
    let now = Utc::now().naive_utc();
    let days_since_join = (now - join_date).num_days();

    let expected_payments = match group.frequency.as_str() {
        "daily" => days_since_join,
        "weekly" => days_since_join / 7,
        "monthly" => days_since_join / 30,
        _ => days_since_join / 7, // Default to weekly
    } as i32;

    // Count actual completed contributions
    let actual_payments = contributions::Entity::find()
        .filter(contributions::Column::MemberId.eq(member_id))
        .filter(contributions::Column::Status.eq("completed"))
        .count(db)
        .await? as i32;

    // Calculate score
    let score = if expected_payments > 0 {
        let percentage = (actual_payments as f64 / expected_payments as f64) * 100.0;
        percentage.round() as i32
    } else {
        100
    };

    // Cap at 100
    Ok(score.min(100))
}

/// Update trust score for a member and record history
pub async fn update_trust_score(
    db: &sea_orm::DatabaseConnection,
    member_id: i32,
    reason: &str,
) -> Result<(i32, i32)> {
    // Get current member
    let member = members::Entity::find_by_id(member_id)
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound)?;

    let old_score = member.trust_score.unwrap_or(100);

    // Calculate new score
    let new_score = calculate_trust_score(db, member_id).await?;
    let change = new_score - old_score;

    // Update member
    let mut member_active: members::ActiveModel = member.into();
    member_active.trust_score = Set(Some(new_score));
    member_active.update(db).await?;

    // Record history
    let history_entry = trust_score_history::ActiveModel {
        member_id: Set(member_id),
        score: Set(new_score),
        change: Set(Some(change)),
        reason: Set(reason.to_string()),
        date: Set(Utc::now().naive_utc()),
        created_at: Set(Some(Utc::now().naive_utc())),
        ..Default::default()
    };
    history_entry.insert(db).await?;

    tracing::info!(
        "Trust score updated for member {}: {} -> {} ({})",
        member_id,
        old_score,
        new_score,
        change
    );

    Ok((new_score, change))
}

/// Update trust score with specific reason type
pub async fn update_trust_score_for_contribution(
    db: &sea_orm::DatabaseConnection,
    member_id: i32,
    was_on_time: bool,
) -> Result<()> {
    let reason = if was_on_time {
        "On-time contribution"
    } else {
        "Late contribution"
    };
    update_trust_score(db, member_id, reason).await?;
    Ok(())
}

pub async fn update_trust_score_for_payout(
    db: &sea_orm::DatabaseConnection,
    member_id: i32,
) -> Result<()> {
    update_trust_score(db, member_id, "Received group payout").await?;
    Ok(())
}

pub async fn update_trust_score_for_missed_payment(
    db: &sea_orm::DatabaseConnection,
    member_id: i32,
) -> Result<()> {
    update_trust_score(db, member_id, "Missed payment").await?;
    Ok(())
}

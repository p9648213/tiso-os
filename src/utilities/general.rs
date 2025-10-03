use axum::http::StatusCode;

use crate::{middlewares::session_mw::UserId, models::error::AppError};

pub fn parse_position(pos: &str) -> Option<(u16, u16)> {
    let parts: Vec<&str> = pos.strip_prefix("item-")?.split('-').collect();
    if parts.len() == 2 {
        let row = parts[0].parse().ok()?;
        let col = parts[1].parse().ok()?;
        Some((row, col))
    } else {
        None
    }
}

pub fn parse_user_id(user_id: UserId) -> Result<i32, AppError> {
    user_id
        .0
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED"))?
        .parse::<i32>()
        .map_err(|err| {
            tracing::error!("Couldn't parse user_id: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })
}

use axum::http::StatusCode;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, state::SessionMap},
};

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
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't parse user_id: {:?}", err),
            )
        })
}

pub fn get_current_dir(session_map: &SessionMap, user_id: i32) -> String {
    let session_map = session_map.pin_owned();
    session_map
        .get(&format!("current-dir-{}", user_id))
        .map(|v| v.to_string())
        .unwrap_or_default()
}

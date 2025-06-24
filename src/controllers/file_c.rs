use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
};
use deadpool_postgres::Pool;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, files_db::File, folders_db::FolderSortType, state::SessionMap},
};

pub async fn update_file_desktop_position(
    Path((file_id, desktop_id, position)): Path<(i32, i32, String)>,
    State(pool): State<Pool>,
    State(session_map): State<SessionMap>,
    Extension(user_id): Extension<UserId>,
) -> Result<(), AppError> {
    let user_id = user_id
        .0
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED"))?
        .parse::<i32>()
        .map_err(|err| {
            tracing::error!("Couldn't parse user_id: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

    let session_map = session_map.pin_owned();

    let current_sort_type = session_map.get(&format!("user-{}-sort-type", user_id));

    let current_sort_type = match current_sort_type {
        Some(sort_type) => match sort_type.as_str() {
            "custom" => Some(FolderSortType::Custom),
            "date_created" => Some(FolderSortType::DateCreated),
            _ => None,
        },
        _ => None,
    };

    File::update_desktop_position(
        file_id,
        desktop_id,
        Some(position),
        &current_sort_type,
        &pool,
    )
    .await?;

    if current_sort_type.is_none() {
        session_map.insert(format!("user-{}-sort-type", user_id), "custom".to_string());
    }

    Ok(())
}

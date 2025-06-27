use axum::{
    Extension,
    extract::{Path, State},
};
use deadpool_postgres::Pool;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, files_db::File, folders_db::FolderSortType, state::SessionMap}, utilities::user_utils::parse_user_id,
};

pub async fn update_file_desktop_position(
    Path((file_id, desktop_id, position)): Path<(i32, i32, String)>,
    State(pool): State<Pool>,
    State(session_map): State<SessionMap>,
    Extension(user_id): Extension<UserId>,
) -> Result<(), AppError> {
    let user_id = parse_user_id(user_id)?;

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
        user_id,
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

pub async fn delete_file(
    Path(file_id): Path<i32>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<(), AppError> {
    let user_id = parse_user_id(user_id)?;

    File::delete_file(file_id, user_id, &pool).await
}

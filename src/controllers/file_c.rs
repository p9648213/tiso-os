use axum::{
    Extension, Form,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use hypertext::Renderable;
use serde::Deserialize;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, file_db::File, folder_db::FolderSortType, state::SessionMap},
    utilities::user_utils::parse_user_id,
    views::txt_v::render_txt_file,
};

#[derive(Deserialize)]
pub struct FileRenameForm {
    pub file_name: String,
}

pub async fn update_file_desktop_position(
    Path((file_id, desktop_id, position)): Path<(i32, i32, String)>,
    State(pool): State<Pool>,
    State(session_map): State<SessionMap>,
    Extension(user_id): Extension<UserId>,
) -> Result<(), AppError> {
    let user_id = parse_user_id(user_id)?;

    let session_map = session_map.pin_owned();

    let current_sort_type = session_map.get(&format!("user-{user_id}-sort-type"));

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
        session_map.insert(format!("user-{user_id}-sort-type"), "custom".to_string());
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

pub async fn rename_file(
    Path((file_type, file_id)): Path<(String, i32)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
    Form(form): Form<FileRenameForm>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    File::rename_file(file_id, user_id, &form.file_name, &pool).await?;

    match file_type.as_str() {
        "txt" => Ok(render_txt_file(file_id, &Some(form.file_name)).render()),
        _ => Err(AppError::new(StatusCode::BAD_REQUEST, "Bad Request")),
    }
}

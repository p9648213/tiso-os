use axum::{
    Extension, Form,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;

use crate::{
    middlewares::session_mw::UserId,
    models::{error::AppError, file_db::File, folder_db::FolderSortType, state::SessionMap},
    utilities::common::parse_user_id,
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

    let file = File::get_file(file_id, user_id, vec!["folder_id"], &pool).await?;

    let rename_result = File::rename_file(
        file_id,
        file.folder_id.unwrap(),
        user_id,
        &form.file_name,
        &pool,
    )
    .await;

    if rename_result.is_err() {
        let file = File::get_file(file_id, user_id, vec!["file_name"], &pool).await?;

        Ok((
            [(
                "HX-Trigger",
                r#"{"message_box":{"type":"error", "title": "Error", "message": "Duplicate name"}}"#,
            )],
            match file_type.as_str() {
                "txt" => Ok(render_txt_file(
                    Some(file_id.to_string()),
                    Some(file.file_name.unwrap()),
                    None,
                )),
                _ => Err(AppError::new(StatusCode::BAD_REQUEST, "Bad Request")),
            },
        ))
    } else {
        Ok((
            [("HX-Trigger", "")],
            match file_type.as_str() {
                "txt" => Ok(render_txt_file(
                    Some(file_id.to_string()),
                    Some(form.file_name),
                    None,
                )),
                _ => Err(AppError::new(StatusCode::BAD_REQUEST, "Bad Request")),
            },
        ))
    }
}

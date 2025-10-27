use axum::{
    Extension, Form,
    extract::{Path, State},
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;

use crate::{
    middlewares::session_mw::UserId,
    models::{
        error::AppError,
        folder_db::{Folder, FolderSortType, FolderType},
        state::SessionMap,
    },
    utilities::common::parse_user_id,
    views::folder_v::{render_folder, render_folder_input},
};

#[derive(Deserialize)]
pub struct FolderRenameForm {
    pub folder_name: String,
}

pub async fn create_folder(
    Path((folder_id, position_id)): Path<(i32, String)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let desktop_position = if position_id == "-1" {
        None
    } else {
        Some(position_id)
    };

    let folder = Folder::create_folder(
        user_id,
        "New Folder",
        FolderType::Normal,
        Some(folder_id),
        desktop_position,
        &pool,
    )
    .await?;

    Ok(render_folder(folder.id.unwrap(), None, None))
}

pub async fn update_folder_desktop_position(
    Path((folder_id, desktop_id, position)): Path<(i32, i32, String)>,
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

    Folder::update_desktop_position(
        folder_id,
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

pub async fn delete_folder(
    Path(folder_id): Path<i32>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<(), AppError> {
    let user_id = parse_user_id(user_id)?;

    Folder::delete_folder(folder_id, user_id, &pool).await
}

pub async fn get_folder_input(
    Path(folder_id): Path<i32>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    let folder = Folder::get_folder(folder_id, user_id, vec!["folder_name"], &pool).await?;

    Ok(render_folder_input(folder_id, &folder.folder_name.unwrap()))
}

pub async fn rename_folder(
    Path(folder_id): Path<i32>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
    Form(form): Form<FolderRenameForm>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    Folder::rename_folder(folder_id, user_id, &form.folder_name, &pool).await?;

    Ok(render_folder(folder_id, Some(form.folder_name), None))
}

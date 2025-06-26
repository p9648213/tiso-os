use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use hypertext::Renderable;

use crate::{
    middlewares::session_mw::UserId,
    models::{
        error::AppError,
        folders_db::{Folder, FolderSortType, FolderType},
        state::SessionMap,
    },
    views::folder_v::render_folder,
};

pub async fn create_folder(
    Path((folder_id, position_id)): Path<(i32, String)>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = user_id
        .0
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED"))?
        .parse::<i32>()
        .map_err(|err| {
            tracing::error!("Couldn't parse user_id: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

    let desktop_position = if position_id == "-1" {
        None
    } else {
        Some(position_id)
    };

    let row = Folder::create_folder(
        user_id,
        "New Folder",
        FolderType::Normal,
        Some(folder_id),
        desktop_position,
        &pool,
    )
    .await?;

    let folder = Folder::try_from(&row, None);

    let folder_id = folder.id.ok_or_else(|| {
        tracing::error!("No id column or value is null");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    Ok(render_folder(folder_id).render())
}

pub async fn update_folder_desktop_position(
    Path((folder_id, desktop_id, position)): Path<(i32, i32, String)>,
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

    Folder::update_desktop_position(
        folder_id,
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

pub async fn delete_folder(
    Path(folder_id): Path<i32>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<(), AppError> {
    user_id
        .0
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "UNAUTHORIZED"))?
        .parse::<i32>()
        .map_err(|err| {
            tracing::error!("Couldn't parse user_id: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

    Folder::delete_folder(folder_id, &pool).await
}

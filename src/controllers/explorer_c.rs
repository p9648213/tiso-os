use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use hypertext::Renderable;
use serde::Deserialize;

use crate::{
    middlewares::session_mw::UserId,
    models::{
        error::AppError,
        folder_db::{Folder, FolderType},
    },
    utilities::user_utils::parse_user_id,
    views::explorer_v::render_explorer,
};

#[derive(Deserialize, Debug)]
pub struct ExplorerPath {
    pub folder_type: FolderType,
    pub folder_id: i32,
    pub height: i32,
    pub width: i32,
}

#[axum::debug_handler]
pub async fn get_explorer(
    Path(explorer_path): Path<ExplorerPath>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    match explorer_path.folder_type {
        FolderType::Root => {
            let row = Folder::get_root_folder(user_id, vec!["id", "folder_name"], &pool).await?;

            let folder = Folder::try_from(&row, None);

            let folder_id = folder.id.ok_or_else(|| {
                tracing::error!("No id column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
            })?;

            let folder_name = folder.folder_name.ok_or_else(|| {
                tracing::error!("No folder_name column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
            })?;

            Ok(render_explorer(
                folder_id,
                folder_name,
                explorer_path.width,
                explorer_path.height,
            )
            .render())
        }
        FolderType::Desktop => {
            let row = Folder::get_desktop_folder(user_id, vec!["id", "folder_name"], &pool).await?;

            let folder = Folder::try_from(&row, None);

            let folder_id = folder.id.ok_or_else(|| {
                tracing::error!("No id column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
            })?;

            let folder_name = folder.folder_name.ok_or_else(|| {
                tracing::error!("No folder_name column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
            })?;

            Ok(render_explorer(
                folder_id,
                folder_name,
                explorer_path.width,
                explorer_path.height,
            )
            .render())
        }
        FolderType::Normal => {
            let row = Folder::get_folder(
                explorer_path.folder_id,
                user_id,
                vec!["id", "folder_name"],
                &pool,
            )
            .await?;
            let folder = Folder::try_from(&row, None);

            let folder_id = folder.id.ok_or_else(|| {
                tracing::error!("No id column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
            })?;

            let folder_name = folder.folder_name.ok_or_else(|| {
                tracing::error!("No folder_name column or value is null");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
            })?;

            Ok(render_explorer(
                folder_id,
                folder_name,
                explorer_path.width,
                explorer_path.height,
            )
            .render())
        }
        _ => Err(AppError::new(StatusCode::NOT_FOUND, "Folder not found")),
    }
}

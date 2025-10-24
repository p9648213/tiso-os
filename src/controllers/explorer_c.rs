use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use deadpool_postgres::Pool;
use serde::Deserialize;

use crate::{
    middlewares::session_mw::UserId,
    models::{
        error::AppError,
        folder_db::{Folder, FolderSortType, FolderType},
        folder_item::FolderItem,
    },
    utilities::general::parse_user_id,
    views::explorer_v::render_explorer_window,
};

#[derive(Deserialize, Debug)]
pub struct ExplorerPath {
    pub folder_type: FolderType,
    pub folder_id: i32,
    pub height: i32,
    pub width: i32,
    pub open_new_task: bool,
    pub previous_folder_id: i32,
}

pub async fn get_explorer_window(
    Path(explorer_path): Path<ExplorerPath>,
    State(pool): State<Pool>,
    Extension(user_id): Extension<UserId>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = parse_user_id(user_id)?;

    match explorer_path.folder_type {
        FolderType::Root => {
            let folder = Folder::get_root_folder(user_id, vec!["id", "folder_name"], &pool).await?;

            let folder_id = folder.id.unwrap();

            let folder_items = FolderItem::get_folder_items(
                folder_id,
                user_id,
                &FolderSortType::DateCreated,
                &pool,
            )
            .await?;

            Ok((
                [(
                    "HX-Trigger",
                    format!(
                        r#"{{"openFile":{{"image":"/assets/images/explorer/folder.svg", "window_id": "explorer-window-{}", "open_new_task": {}, "previous_folder_id": {}}}}}"#,
                        folder_id, explorer_path.open_new_task, explorer_path.previous_folder_id
                    ),
                )],
                render_explorer_window(
                    folder_id,
                    folder.folder_name.unwrap(),
                    explorer_path.width,
                    explorer_path.height,
                    &folder_items,
                ),
            ))
        }
        FolderType::Desktop => {
            let folder =
                Folder::get_desktop_folder(user_id, vec!["id", "folder_name"], &pool).await?;

            let folder_id = folder.id.unwrap();

            let folder_items = FolderItem::get_folder_items(
                folder_id,
                user_id,
                &FolderSortType::DateCreated,
                &pool,
            )
            .await?;

            Ok((
                [(
                    "HX-Trigger",
                    format!(
                        r#"{{"openFile":{{"image":"/assets/images/explorer/folder.svg", "window_id": "explorer-window-{}", "open_new_task": {}, "previous_folder_id": {}}}}}"#,
                        folder_id, explorer_path.open_new_task, explorer_path.previous_folder_id
                    ),
                )],
                render_explorer_window(
                    folder_id,
                    folder.folder_name.unwrap(),
                    explorer_path.width,
                    explorer_path.height,
                    &folder_items,
                ),
            ))
        }
        FolderType::Normal => {
            let folder = Folder::get_folder(
                explorer_path.folder_id,
                user_id,
                vec!["id", "folder_name"],
                &pool,
            )
            .await?;

            let folder_id = folder.id.unwrap();

            let folder_items = FolderItem::get_folder_items(
                folder_id,
                user_id,
                &FolderSortType::DateCreated,
                &pool,
            )
            .await?;

            Ok((
                [(
                    "HX-Trigger",
                    format!(
                        r#"{{"openFile":{{"image":"/assets/images/explorer/folder.svg", "window_id": "explorer-window-{}", "open_new_task": {}, "previous_folder_id": {}}}}}"#,
                        folder_id, explorer_path.open_new_task, explorer_path.previous_folder_id
                    ),
                )],
                render_explorer_window(
                    folder_id,
                    folder.folder_name.unwrap(),
                    explorer_path.width,
                    explorer_path.height,
                    &folder_items,
                ),
            ))
        }
        _ => Err(AppError::new(StatusCode::NOT_FOUND, "Folder not found")),
    }
}

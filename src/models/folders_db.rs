use axum::http::StatusCode;
use deadpool_postgres::Pool;
use time::OffsetDateTime;
use tokio_postgres::{
    Row,
    types::{FromSql, ToSql},
};

use crate::{models::error::AppError, utilities::postgres::DbExecutor};

#[derive(Debug, ToSql, FromSql, Clone)]
#[postgres(name = "foldertype")]
pub enum FolderType {
    Normal,
    Root,
    Desktop,
}

pub struct Folder {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub folder_name: Option<String>,
    pub folder_type: Option<FolderType>,
    pub parent_folder_id: Option<i32>,
    pub created_at: Option<OffsetDateTime>,
}

impl Folder {
    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row
            .try_get(format!("{}id", prefix).as_str())
            .unwrap_or(None);
        let user_id: Option<i32> = row
            .try_get(format!("{}user_id", prefix).as_str())
            .unwrap_or(None);
        let folder_name: Option<String> = row
            .try_get(format!("{}folder_name", prefix).as_str())
            .unwrap_or(None);
        let folder_type: Option<FolderType> = row
            .try_get(format!("{}folder_type", prefix).as_str())
            .unwrap_or(None);
        let parent_folder_id: Option<i32> = row
            .try_get(format!("{}parent_folder_id", prefix).as_str())
            .unwrap_or(None);
        let created_at: Option<OffsetDateTime> = row
            .try_get(format!("{}created_at", prefix).as_str())
            .unwrap_or(None);

        Self {
            id,
            user_id,
            folder_name,
            folder_type,
            parent_folder_id,
            created_at,
        }
    }

    pub async fn get_desktop_folders(
        user_id: i32,
        columns: Vec<&str>,
        pool: &Pool,
    ) -> Result<Row, AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let columns = columns.join(",");

        client
            .query_one(
                &format!(
                    "SELECT {} FROM folders WHERE user_id = $1 AND folder_type = $2",
                    columns
                ),
                &[&user_id, &FolderType::Desktop],
            )
            .await
    }

    pub async fn create_folder(
        user_id: i32,
        folder_name: &str,
        folder_type: FolderType,
        parent_folder_id: Option<i32>,
        pool: &Pool,
    ) -> Result<Row, AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        client.query_one(
            "INSERT INTO folders (user_id, folder_name, folder_type, parent_folder_id) VALUES ($1, $2, $3, $4) RETURNING id",
            &[&user_id, &folder_name, &folder_type, &parent_folder_id],
        )
        .await
    }
}

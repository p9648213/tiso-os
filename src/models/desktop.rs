use axum::http::StatusCode;
use deadpool_postgres::Pool;
use time::OffsetDateTime;
use tokio_postgres::Row;

use crate::models::{error::AppError, folder_db::FolderSortType};

#[derive(Debug)]
pub enum ItemType {
    File,
    Folder,
}

#[derive(Debug)]
pub struct DesktopItem {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub item_type: Option<ItemType>,
    pub desktop_position: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}

impl DesktopItem {
    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row
            .try_get(format!("{prefix}id").as_str())
            .unwrap_or(None);
        let user_id: Option<i32> = row
            .try_get(format!("{prefix}user_id").as_str())
            .unwrap_or(None);
        let name: Option<String> = row
            .try_get(format!("{prefix}name").as_str())
            .unwrap_or(None);
        let item_type: Option<String> = row
            .try_get(format!("{prefix}item_type").as_str())
            .unwrap_or(None);
        let desktop_position: Option<String> = row
            .try_get(format!("{prefix}desktop_position").as_str())
            .unwrap_or(None);
        let created_at: Option<OffsetDateTime> = row
            .try_get(format!("{prefix}created_at").as_str())
            .unwrap_or(None);

        let item_type = match item_type.as_deref() {
            Some("file") => Some(ItemType::File),
            Some("folder") => Some(ItemType::Folder),
            _ => None,
        };

        Self {
            id,
            user_id,
            name,
            item_type,
            desktop_position,
            created_at,
        }
    }

    pub fn try_from_vec(rows: Vec<Row>, prefix: Option<&str>) -> Vec<Self> {
        rows.into_iter()
            .map(|row| Self::try_from(&row, prefix))
            .collect()
    }

    pub async fn get_desktop_items(
        desktop_id: i32,
        user_id: i32,
        sort_type: &FolderSortType,
        pool: &Pool,
    ) -> Result<Vec<Row>, AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let sql = 
            "SELECT * FROM (
                SELECT id, user_id, file_name AS name, 'file' AS item_type, file_type, desktop_position, created_at
                FROM file WHERE folder_id = $1 AND user_id = $2
                UNION
                SELECT id, user_id, folder_name AS name, 'folder' AS item_type, NULL AS file_type, desktop_position, created_at
                FROM folder WHERE parent_folder_id = $1 AND user_id = $2
            ) AS combined";

        let sql = match sort_type {
            FolderSortType::Custom => sql,
            FolderSortType::DateCreated => &format!("{sql} ORDER BY created_at ASC"),
        };

        client.query(sql, &[&desktop_id, &user_id]).await.map_err(|error| {
            tracing::error!("Couldn't query postgres: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })
    }
}

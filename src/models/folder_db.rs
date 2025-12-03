use axum::http::StatusCode;
use deadpool_postgres::Pool;
use serde::Deserialize;
use time::OffsetDateTime;
use tokio_postgres::{
    Row,
    types::{FromSql, ToSql},
};

use crate::{models::error::AppError, utilities::postgres::DbExecutor};

#[derive(Debug, ToSql, FromSql, Clone, Deserialize, PartialEq, Eq)]
#[postgres(name = "foldertype")]
pub enum FolderType {
    Normal,
    Root,
    Desktop,
    Taskbar,
}

#[derive(Debug, ToSql, FromSql, Clone, PartialEq, Eq)]
#[postgres(name = "foldersorttype")]
pub enum FolderSortType {
    Custom,
    DateCreated,
}

#[derive(Debug)]
pub struct Folder {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub folder_name: Option<String>,
    pub folder_type: Option<FolderType>,
    pub sort_type: Option<FolderSortType>,
    pub desktop_position: Option<String>,
    pub parent_folder_id: Option<i32>,
    pub path: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}

impl Folder {
    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row.try_get(format!("{prefix}id").as_str()).unwrap_or(None);
        let user_id: Option<i32> = row
            .try_get(format!("{prefix}user_id").as_str())
            .unwrap_or(None);
        let folder_name: Option<String> = row
            .try_get(format!("{prefix}folder_name").as_str())
            .unwrap_or(None);
        let folder_type: Option<FolderType> = row
            .try_get(format!("{prefix}folder_type").as_str())
            .unwrap_or(None);
        let sort_type: Option<FolderSortType> = row
            .try_get(format!("{prefix}sort_type").as_str())
            .unwrap_or(None);
        let desktop_position: Option<String> = row
            .try_get(format!("{prefix}desktop_position").as_str())
            .unwrap_or(None);
        let parent_folder_id: Option<i32> = row
            .try_get(format!("{prefix}parent_folder_id").as_str())
            .unwrap_or(None);
        let path: Option<String> = row
            .try_get(format!("{prefix}path").as_str())
            .unwrap_or(None);
        let created_at: Option<OffsetDateTime> = row
            .try_get(format!("{prefix}created_at").as_str())
            .unwrap_or(None);

        Self {
            id,
            user_id,
            folder_name,
            folder_type,
            sort_type,
            desktop_position,
            parent_folder_id,
            path,
            created_at,
        }
    }

    pub fn try_from_vec(rows: Vec<Row>, prefix: Option<&str>) -> Vec<Self> {
        rows.into_iter()
            .map(|row| Self::try_from(&row, prefix))
            .collect()
    }

    pub async fn get_folder(
        folder_id: i32,
        user_id: i32,
        columns: Vec<&str>,
        pool: &Pool,
    ) -> Result<Folder, AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let columns = columns.join(",");

        let row = client
            .query_one(
                &format!("SELECT {columns} FROM folder WHERE id = $1 AND user_id = $2"),
                &[&folder_id, &user_id],
            )
            .await?;

        Ok(Self::try_from(&row, None))
    }

    pub async fn get_desktop_folder(
        user_id: i32,
        columns: Vec<&str>,
        pool: &Pool,
    ) -> Result<Folder, AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let columns = columns.join(",");

        let row = client
            .query_one(
                &format!("SELECT {columns} FROM folder WHERE user_id = $1 AND folder_type = $2"),
                &[&user_id, &FolderType::Desktop],
            )
            .await?;

        Ok(Self::try_from(&row, None))
    }

    pub async fn get_root_folder(
        user_id: i32,
        columns: Vec<&str>,
        pool: &Pool,
    ) -> Result<Folder, AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let columns = columns.join(",");

        let row = client
            .query_one(
                &format!("SELECT {columns} FROM folder WHERE user_id = $1 AND folder_type = $2"),
                &[&user_id, &FolderType::Root],
            )
            .await?;

        Ok(Self::try_from(&row, None))
    }

    pub async fn get_taskbar_folder(
        user_id: i32,
        columns: Vec<&str>,
        pool: &Pool,
    ) -> Result<Folder, AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let columns = columns.join(",");

        let row = client
            .query_one(
                &format!("SELECT {columns} FROM folder WHERE user_id = $1 AND folder_type = $2"),
                &[&user_id, &FolderType::Taskbar],
            )
            .await?;

        Ok(Self::try_from(&row, None))
    }

    pub async fn create_folder(
        user_id: i32,
        mut folder_name: String,
        folder_type: FolderType,
        parent_folder_id: Option<i32>,
        desktop_position: Option<String>,
        mut path: String,
        pool: &Pool,
    ) -> Result<Folder, AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let initial_folder_name = folder_name.clone();

        if let Some(parent_folder_id) = parent_folder_id {
            let sql = "
                SELECT * FROM (
                    SELECT id from folder WHERE parent_folder_id = $1 AND folder_name = $2
                    UNION
                    SELECT id from file WHERE folder_id = $1 AND file_name = $2
                ) AS combined
            ";

            let mut row = client
                .query(sql, &[&parent_folder_id, &folder_name])
                .await?;

            while !row.is_empty() {
                let random_numb = rand::random_range(0..1000);
                folder_name = format!("{} {}", initial_folder_name, random_numb);
                row = client
                    .query(sql, &[&parent_folder_id, &folder_name])
                    .await?;
            }
        }

        path = format!("{}/{}", path, folder_name);

        let row = client.query_one(
            "INSERT INTO folder (user_id, folder_name, folder_type, parent_folder_id, desktop_position, path) 
                    VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, folder_name",
            &[&user_id, &folder_name, &folder_type, &parent_folder_id, &desktop_position, &path],
        )
        .await?;

        Ok(Self::try_from(&row, None))
    }

    pub async fn update_desktop_position(
        id: i32,
        desktop_id: i32,
        user_id: i32,
        desktop_position: Option<String>,
        current_sort_type: &Option<FolderSortType>,
        pool: &Pool,
    ) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let rows = client
            .execute(
                "UPDATE folder SET desktop_position = $1 WHERE id = $2 AND user_id = $3",
                &[&desktop_position, &id, &user_id],
            )
            .await?;

        if rows == 0 {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error updating folder desktop position",
            ));
        }

        let should_update_sort_type = current_sort_type
            .as_ref()
            .map(|t| *t != FolderSortType::Custom)
            .unwrap_or(true);

        if should_update_sort_type {
            let rows = client
                .execute(
                    "UPDATE folder SET sort_type = $1 WHERE id = $2",
                    &[&FolderSortType::Custom, &desktop_id],
                )
                .await?;

            if rows == 0 {
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error updating folder sort type",
                ));
            }
        }

        Ok(())
    }

    pub async fn delete_folder(id: i32, user_id: i32, pool: &Pool) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let rows = client
            .execute(
                "DELETE FROM folder WHERE id = $1 AND user_id = $2",
                &[&id, &user_id],
            )
            .await?;

        if rows == 0 {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error deleting folder",
            ));
        }

        Ok(())
    }

    pub async fn rename_folder(
        id: i32,
        user_id: i32,
        folder_name: &str,
        pool: &Pool,
    ) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let sql = "
            WITH parent AS (
                SELECT parent_folder_id FROM folder WHERE id = $1
            ) SELECT * FROM (
                SELECT id FROM folder WHERE parent_folder_id = (SELECT parent_folder_id FROM parent) AND folder_name = $2
                UNION
                SELECT id FROM file WHERE folder_id = (SELECT parent_folder_id FROM parent) AND file_name = $2
            ) AS combined;
        ";

        let row = client.query(sql, &[&id, &folder_name]).await?;

        if !row.is_empty() {
            return Err(AppError::new(StatusCode::BAD_REQUEST, "Duplicate name"));
        }

        let row = client
            .execute(
                "UPDATE folder SET folder_name = $1 WHERE id = $2 AND user_id = $3",
                &[&folder_name, &id, &user_id],
            )
            .await?;

        if row == 0 {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error updating folder name",
            ));
        }

        Ok(())
    }
}

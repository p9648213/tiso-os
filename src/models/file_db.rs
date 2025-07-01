use axum::http::StatusCode;
use deadpool_postgres::Pool;
use postgres_types::{FromSql, ToSql};
use time::OffsetDateTime;
use tokio_postgres::Row;

use crate::{
    models::{error::AppError, folder_db::FolderSortType},
    utilities::postgres::DbExecutor,
};

#[derive(Debug, ToSql, FromSql, Clone)]
#[postgres(name = "filetype")]
pub enum FileType {
    Txt,
    Calculator,
}

pub struct File {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub folder_id: Option<i32>,
    pub file_name: Option<String>,
    pub file_type: Option<FileType>,
    pub desktop_position: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}

impl File {
    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row
            .try_get(format!("{}id", prefix).as_str())
            .unwrap_or(None);
        let user_id: Option<i32> = row
            .try_get(format!("{}user_id", prefix).as_str())
            .unwrap_or(None);
        let folder_id: Option<i32> = row
            .try_get(format!("{}folder_id", prefix).as_str())
            .unwrap_or(None);
        let file_name: Option<String> = row
            .try_get(format!("{}file_name", prefix).as_str())
            .unwrap_or(None);
        let file_type: Option<FileType> = row
            .try_get(format!("{}file_type", prefix).as_str())
            .unwrap_or(None);
        let desktop_position: Option<String> = row
            .try_get(format!("{}desktop_position", prefix).as_str())
            .unwrap_or(None);
        let created_at: Option<OffsetDateTime> = row
            .try_get(format!("{}created_at", prefix).as_str())
            .unwrap_or(None);

        Self {
            id,
            user_id,
            folder_id,
            file_name,
            file_type,
            desktop_position,
            created_at,
        }
    }

    pub fn try_from_vec(rows: Vec<Row>, prefix: Option<&str>) -> Vec<Self> {
        rows.into_iter()
            .map(|row| Self::try_from(&row, prefix))
            .collect()
    }

    pub async fn get_file(
        id: i32,
        user_id: i32,
        columns: Vec<&str>,
        pool: &Pool,
    ) -> Result<Row, AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let columns = columns.join(",");

        let row = client
            .query_one(
                &format!(
                    "SELECT {} FROM file WHERE id = $1 AND user_id = $2",
                    columns
                ),
                &[&id, &user_id],
            )
            .await?;

        Ok(row)
    }

    pub async fn create_file(
        user_id: i32,
        folder_id: i32,
        file_name: &str,
        file_type: FileType,
        desktop_position: Option<String>,
        pool: &Pool,
    ) -> Result<Row, AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        client
            .query_one(
                "INSERT INTO file (user_id, folder_id, file_name, file_type, desktop_position) 
                    VALUES ($1, $2, $3, $4, $5) RETURNING id",
                &[
                    &user_id,
                    &folder_id,
                    &file_name,
                    &file_type,
                    &desktop_position,
                ],
            )
            .await
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
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let rows = client
            .execute(
                "UPDATE file SET desktop_position = $1 WHERE id = $2 AND user_id = $3",
                &[&desktop_position, &id, &user_id],
            )
            .await?;

        if rows == 0 {
            tracing::error!("Error updating file desktop position");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
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
                tracing::error!("Error updating folder sort type");
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server Error",
                ));
            }
        }

        Ok(())
    }

    pub async fn delete_file(id: i32, user_id: i32, pool: &Pool) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let rows = client
            .execute(
                "DELETE FROM file WHERE id = $1 AND user_id = $2",
                &[&id, &user_id],
            )
            .await?;

        if rows == 0 {
            tracing::error!("Error deleting file");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            ));
        }

        Ok(())
    }

    pub async fn rename_file(
        id: i32,
        user_id: i32,
        file_name: &str,
        pool: &Pool,
    ) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let row = client
            .execute(
                "UPDATE file SET file_name = $1 WHERE id = $2 AND user_id = $3",
                &[&file_name, &id, &user_id],
            )
            .await?;

        if row == 0 {
            tracing::error!("Error updating file name");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            ));
        }

        Ok(())
    }
}

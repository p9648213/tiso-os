use axum::http::StatusCode;
use deadpool_postgres::Pool;
use time::OffsetDateTime;
use tokio_postgres::Row;

use crate::{
    models::{error::AppError, folders_db::FolderSortType},
    utilities::postgres::DbExecutor,
};

pub struct File {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub folder_id: Option<i32>,
    pub file_name: Option<String>,
    pub execute_path: Option<String>,
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
        let execute_path: Option<String> = row
            .try_get(format!("{}execute_path", prefix).as_str())
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
            execute_path,
            desktop_position,
            created_at,
        }
    }

    pub fn try_from_vec(rows: Vec<Row>, prefix: Option<&str>) -> Vec<Self> {
        rows.into_iter()
            .map(|row| Self::try_from(&row, prefix))
            .collect()
    }

    pub async fn get_files_by_folder_id(
        folder_id: i32,
        columns: Vec<&str>,
        pool: &Pool,
    ) -> Result<Vec<Row>, AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let columns = columns.join(",");

        client
            .query(
                &format!("SELECT {} FROM files WHERE folder_id = $1", columns),
                &[&folder_id],
            )
            .await
            .map_err(|error| {
                tracing::error!("Couldn't query postgres: {:?}", error);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
            })
    }

    pub async fn create_file(
        user_id: i32,
        folder_id: i32,
        file_name: &str,
        execute_path: &str,
        desktop_position: Option<String>,
        pool: &Pool,
    ) -> Result<Row, AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        client
            .query_one(
                "INSERT INTO files (user_id, folder_id, file_name, execute_path, desktop_position) 
                    VALUES ($1, $2, $3, $4, $5) RETURNING id",
                &[
                    &user_id,
                    &folder_id,
                    &file_name,
                    &execute_path,
                    &desktop_position,
                ],
            )
            .await
    }

    pub async fn update_desktop_position(
        id: i32,
        desktop_id: i32,
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
                "UPDATE files SET desktop_position = $1 WHERE id = $2",
                &[&desktop_position, &id],
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
                    "UPDATE folders SET sort_type = $1 WHERE id = $2",
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

    pub async fn delete_file(id: i32, pool: &Pool) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let rows = client
            .execute("DELETE FROM files WHERE id = $1", &[&id])
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
}

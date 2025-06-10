use time::OffsetDateTime;
use tokio_postgres::Row;

use crate::{models::error::AppError, utilities::postgres::query_one};

pub struct File {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub folder_id: Option<i32>,
    pub file_name: Option<String>,
    pub execute_path: Option<String>,
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
        let created_at: Option<OffsetDateTime> = row
            .try_get(format!("{}created_at", prefix).as_str())
            .unwrap_or(None);

        Self {
            id,
            user_id,
            folder_id,
            file_name,
            execute_path,
            created_at,
        }
    }

    pub async fn create_file(
        user_id: i32,
        folder_id: i32,
        file_name: &str,
        execute_path: &str,
        pool: &deadpool_postgres::Pool,
    ) -> Result<Row, AppError> {
        query_one(
            "INSERT INTO files (user_id, folder_id, file_name, execute_path) VALUES ($1, $2, $3, $4) RETURNING id",
            &[&user_id, &folder_id, &file_name, &execute_path],
            pool,
        )
        .await
    }
}
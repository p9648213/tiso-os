use axum::http::StatusCode;
use deadpool_postgres::Pool;
use time::OffsetDateTime;
use tokio_postgres::Row;

use crate::{models::error::AppError, utilities::postgres::DbExecutor};

#[derive(Debug, Clone)]
pub struct Setting {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub background: Option<Vec<u8>>,
    pub created_at: Option<OffsetDateTime>,
}

impl Setting {
    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row.try_get(format!("{prefix}id").as_str()).unwrap_or(None);
        let user_id: Option<i32> = row
            .try_get(format!("{prefix}user_id").as_str())
            .unwrap_or(None);
        let background: Option<Vec<u8>> = row
            .try_get(format!("{prefix}background").as_str())
            .unwrap_or(None);
        let created_at: Option<OffsetDateTime> = row
            .try_get(format!("{prefix}created_at").as_str())
            .unwrap_or(None);

        Self {
            id,
            user_id,
            background,
            created_at,
        }
    }

    pub async fn get_setting_by_user_id(
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
                &format!(r#"SELECT {columns} FROM "setting" WHERE user_id = $1"#),
                &[&user_id],
            )
            .await
    }

    pub async fn update_background_by_user_id(
        user_id: i32,
        background: Option<Vec<u8>>,
        pool: &Pool,
    ) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let row = client
            .execute(
                "UPDATE setting SET background = $1 WHERE user_id = $2",
                &[&background, &user_id],
            )
            .await?;

        if row == 0 {
            tracing::error!("Error updating background");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            ));
        }

        Ok(())
    }
}

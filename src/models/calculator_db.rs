use axum::http::StatusCode;
use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::{models::error::AppError, utilities::postgres::DbExecutor};

pub struct Calculator {
    pub id: Option<i32>,
    pub file_id: Option<i32>,
}

impl Calculator {
    pub fn try_from(row: &Row) -> Self {
        let id: Option<i32> = row.try_get("id").unwrap_or(None);
        let file_id: Option<i32> = row.try_get("file_id").unwrap_or(None);

        Self { id, file_id }
    }

    pub fn try_from_vec(rows: Vec<Row>) -> Vec<Self> {
        rows.into_iter().map(|row| Self::try_from(&row)).collect()
    }

    pub async fn create_calculator(file_id: i32, pool: &Pool) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let rows = client
            .execute("INSERT INTO calculator (file_id) VALUES ($1)", &[&file_id])
            .await?;

        if rows == 0 {
            tracing::error!("Error creating calculator");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            ));
        }

        Ok(())
    }
}

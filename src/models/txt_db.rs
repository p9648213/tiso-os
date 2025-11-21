use axum::http::StatusCode;
use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::{models::error::AppError, utilities::postgres::DbExecutor};

pub struct Txt {
    pub id: Option<i32>,
    pub file_id: Option<i32>,
}

impl Txt {
    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row.try_get(format!("{prefix}id").as_str()).unwrap_or(None);
        let file_id: Option<i32> = row
            .try_get(format!("{prefix}file_id").as_str())
            .unwrap_or(None);

        Self { id, file_id }
    }

    pub fn try_from_vec(rows: Vec<Row>, prefix: Option<&str>) -> Vec<Self> {
        rows.into_iter()
            .map(|row| Self::try_from(&row, prefix))
            .collect()
    }

    pub async fn create_txt(file_id: i32, pool: &Pool) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, &format!("Couldn't get postgres client: {:?}", error))
        })?;

        let rows = client
            .execute("INSERT INTO txt (file_id) VALUES ($1)", &[&file_id])
            .await?;

        if rows == 0 {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error creating calculator",
            ));
        }

        Ok(())
    }
}

use axum::http::StatusCode;
use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::{
    models::{error::AppError, file_db::File, web_builder_db::WebBuilder},
    utilities::postgres::DbExecutor,
};

pub struct WebBuilderWindow {
    pub web_builder: WebBuilder,
    pub file: File,
}

impl WebBuilderWindow {
    pub fn try_from(row: &Row) -> Self {
        Self {
            web_builder: WebBuilder::try_from(row, Some("web_builder_")),
            file: File::try_from(row, Some("file_")),
        }
    }

    pub fn try_from_vec(rows: Vec<Row>) -> Vec<Self> {
        rows.into_iter().map(|row| Self::try_from(&row)).collect()
    }

    pub async fn get_web_builder_window(
        file_id: i32,
        user_id: i32,
        web_builder_columns: Vec<&str>,
        file_columns: Vec<&str>,
        pool: &Pool,
    ) -> Result<WebBuilderWindow, AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let mut columns = vec![];

        web_builder_columns.iter().for_each(|col| {
            columns.push(format!("web_builder.{col} AS web_builder_{col}"));
        });

        file_columns.iter().for_each(|col| {
            columns.push(format!("file.{col} AS file_{col}"));
        });

        let columns = columns.join(",");

        let row = client
            .query_one(
                &format!(
                    "SELECT {columns} FROM web_builder JOIN file ON web_builder.file_id = file.id WHERE file.id = $1 AND file.user_id = $2"
                ),
                &[&file_id, &user_id],
            )
            .await?;

        Ok(Self::try_from(&row))
    }
}

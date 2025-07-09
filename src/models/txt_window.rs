use axum::http::StatusCode;
use deadpool_postgres::Pool;
use tokio_postgres::Row;

use crate::{
    models::{error::AppError, file_db::File, txt_db::Txt},
    utilities::postgres::DbExecutor,
};

pub struct TxtWindow {
    pub txt: Txt,
    pub file: File,
}

impl TxtWindow {
    pub fn try_from(row: &Row) -> Self {
        Self {
            txt: Txt::try_from(row, Some("txt_")),
            file: File::try_from(row, Some("file_")),
        }
    }

    pub fn try_from_vec(rows: Vec<Row>) -> Vec<Self> {
        rows.into_iter().map(|row| Self::try_from(&row)).collect()
    }

    pub async fn get_txt_window(
        file_id: i32,
        user_id: i32,
        txt_columns: Vec<&str>,
        file_columns: Vec<&str>,
        pool: &Pool,
    ) -> Result<Row, AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let mut columns = vec![];

        txt_columns.iter().for_each(|col| {
            columns.push(format!("txt.{col} AS txt_{col}"));
        });

        file_columns.iter().for_each(|col| {
            columns.push(format!("file.{col} AS file_{col}"));
        });

        let columns = columns.join(",");

        client
            .query_one(
                &format!(
                    "SELECT {columns} FROM txt JOIN file ON txt.file_id = file.id WHERE file.id = $1 AND file.user_id = $2"
                ),
                &[&file_id, &user_id],
            )
            .await
    }
}

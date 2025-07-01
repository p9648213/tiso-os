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
        mut txt_columns: Vec<String>,
        mut file_columns: Vec<String>,
        pool: &Pool,
    ) -> Result<Row, AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        txt_columns.iter_mut().for_each(|col| {
          *col = format!("txt.{} AS txt_{}", col, col);
        });

        file_columns.iter_mut().for_each(|col| {
          *col = format!("file.{} AS file_{}", col, col);
        });

        let columns = [txt_columns, file_columns].concat().join(",");

        client
            .query_one(
                &format!(
                    "SELECT {} FROM txt, file WHERE txt.file_id = file.id AND file.id = $1",
                    columns
                ),
                &[&file_id],
            )
            .await
    }
}

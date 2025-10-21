use std::collections::HashMap;

use axum::http::StatusCode;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;

use crate::{models::error::AppError, utilities::postgres::DbExecutor};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub tag: String,
    pub children: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomTree {
    pub html_node: String,
    pub body_node: String,
    pub nodes: HashMap<String, Node>,
}

pub struct WebBuilder {
    pub id: Option<i32>,
    pub file_id: Option<i32>,
    pub name: Option<String>,
    pub data: Option<Value>,
}

impl WebBuilder {
    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row.try_get(format!("{prefix}id").as_str()).unwrap_or(None);
        let file_id: Option<i32> = row
            .try_get(format!("{prefix}file_id").as_str())
            .unwrap_or(None);
        let name: Option<String> = row
            .try_get(format!("{prefix}name").as_str())
            .unwrap_or(None);
        let data: Option<Value> = row
            .try_get(format!("{prefix}data").as_str())
            .unwrap_or(None);

        Self {
            id,
            file_id,
            name,
            data,
        }
    }

    pub fn try_from_vec(rows: Vec<Row>, prefix: Option<&str>) -> Vec<Self> {
        rows.into_iter()
            .map(|row| Self::try_from(&row, prefix))
            .collect()
    }

    pub async fn create_web_builder(file_id: i32, name: &str, pool: &Pool) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let rows = client
            .execute(
                "INSERT INTO web_builder (file_id, name) VALUES ($1, $2)",
                &[&file_id, &name],
            )
            .await?;

        if rows == 0 {
            tracing::error!("Error creating web builder");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            ));
        }

        Ok(())
    }
}

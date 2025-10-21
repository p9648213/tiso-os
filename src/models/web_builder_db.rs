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

    pub async fn insert_node(
        builder_id: i32,
        user_id: i32,
        pool: &Pool,
        insert_node_id: String,
        parent_node_id: String,
        update_node: Node,
    ) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let node_json = serde_json::to_value(&update_node).map_err(|e| {
            tracing::error!("Failed to serialize node: {:?}", e);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Serialization error")
        })?;

        let rows = client
            .execute(
                "UPDATE web_builder
                SET data = jsonb_set(
                    jsonb_set(
                        data,
                        $1::text[],
                        $2
                    ),
                    $3::text[],
                    (data->'nodes'->$4->'children' || jsonb_build_array($5))
                )
                FROM file
                WHERE web_builder.builder_id = file.builder_id
                AND web_builder.id = $6
                AND file.user_id = $7;",
                &[
                    &vec![format!("nodes"), insert_node_id.clone()],
                    &node_json,
                    &vec![
                        format!("nodes"),
                        parent_node_id.clone(),
                        format!("children"),
                    ],
                    &parent_node_id,
                    &insert_node_id,
                    &builder_id,
                    &user_id,
                ],
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

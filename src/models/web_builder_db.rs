use std::collections::HashMap;

use axum::http::StatusCode;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_postgres::Row;

use crate::{
    models::error::AppError,
    utilities::{common::collect_descendants, postgres::DbExecutor},
};

fn default_attributes() -> Value {
    Value::Object(serde_json::Map::new())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub tag: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub children: Vec<String>,
    #[serde(default = "default_attributes")]
    pub attributes: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomTree {
    pub html_node: String,
    pub body_node: String,
    pub nodes: HashMap<String, Node>,
}

#[derive(Debug)]
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
        insert_node_id: String,
        parent_node_id: String,
        update_node: Node,
        pool: &Pool,
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
                    (data->'nodes'->$4->'children' || jsonb_build_array(to_jsonb($5::text)))
                )
                FROM file
                WHERE web_builder.file_id = file.id
                AND web_builder.id = $6
                AND file.user_id = $7;",
                &[
                    &vec![String::from("nodes"), insert_node_id.clone()],
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
            tracing::error!("Error insert node");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            ));
        }

        Ok(())
    }

    pub async fn insert_nodes_to_body(
        builder_id: i32,
        user_id: i32,
        insert_nodes: HashMap<String, Node>,
        root_node_ids: Vec<String>,
        pool: &Pool,
    ) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let nodes_json = serde_json::to_value(&insert_nodes).map_err(|e| {
            tracing::error!("Failed to serialize nodes: {:?}", e);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Serialization error")
        })?;

        let root_node_ids_json = serde_json::to_value(&root_node_ids).map_err(|e| {
            tracing::error!("Failed to serialize root_node_ids: {:?}", e);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Serialization error")
        })?;

        let rows = client
            .execute(
                "UPDATE web_builder 
                SET data = jsonb_set(
                    data || jsonb_build_object('nodes', data->'nodes' || $1::jsonb),
                    array['nodes', data->>'body_node', 'children']::text[],
                    (data#>array['nodes', data->>'body_node', 'children']::text[] || $2::jsonb)
                )
                FROM file
                WHERE web_builder.file_id = file.id
                AND web_builder.id = $3
                AND file.user_id = $4;",
                &[&nodes_json, &root_node_ids_json, &builder_id, &user_id],
            )
            .await?;

        if rows == 0 {
            tracing::error!("Error insert nodes");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            ));
        }

        Ok(())
    }

    pub async fn edit_node(
        builder_id: i32,
        user_id: i32,
        edit_node_id: String,
        update_node: &Node,
        pool: &Pool,
    ) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let node_json = serde_json::to_value(update_node).map_err(|e| {
            tracing::error!("Failed to serialize node: {:?}", e);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Serialization error")
        })?;

        let rows = client
            .execute(
                "UPDATE web_builder
            SET data = jsonb_set(
                data,
                $1::text[],
                $2
            )
            FROM file
            WHERE web_builder.file_id = file.id
            AND web_builder.id = $3
            AND file.user_id = $4;",
                &[
                    &vec![String::from("nodes"), edit_node_id.clone()],
                    &node_json,
                    &builder_id,
                    &user_id,
                ],
            )
            .await?;

        if rows == 0 {
            tracing::error!("Error edit node");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            ));
        }

        Ok(())
    }

    pub async fn delete_node(
        builder_id: i32,
        user_id: i32,
        delete_node_id: String,
        pool: &Pool,
    ) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let row = client
            .query_one(
                "SELECT data 
                FROM web_builder JOIN file 
                ON web_builder.file_id = file.id
                WHERE web_builder.id = $1
                AND file.user_id = $2;",
                &[&builder_id, &user_id],
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch builder data: {:?}", e);
                AppError::new(StatusCode::NOT_FOUND, "Builder not found")
            })?;

        let data: Value = row.get("data");

        let nodes = data
            .get("nodes")
            .and_then(|n| n.as_object())
            .ok_or_else(|| {
                tracing::error!("Invalid nodes structure");
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Invalid data structure")
            })?;

        if !nodes.contains_key(&delete_node_id) {
            return Err(AppError::new(StatusCode::NOT_FOUND, "Not Found"));
        }

        let mut to_delete = std::collections::HashSet::new();
        collect_descendants(&delete_node_id, nodes, &mut to_delete);
        let all_ids: Vec<String> = to_delete.into_iter().collect();

        let rows = client
        .execute(
            "UPDATE web_builder
            SET data = jsonb_set(
                data,
                '{nodes}',
                (
                    SELECT jsonb_object_agg(key, value)
                    FROM (
                        SELECT key,
                            CASE 
                                WHEN value ? 'children' THEN jsonb_set(
                                    value,
                                    '{children}',
                                    (
                                        SELECT COALESCE(jsonb_agg(child_id), '[]'::jsonb)
                                        FROM jsonb_array_elements_text(value->'children') AS child_id
                                        WHERE child_id <> ALL($1::text[])
                                    )
                                )
                                ELSE value
                            END as value
                        FROM jsonb_each(data->'nodes')
                        WHERE key <> ALL($1::text[])
                    ) AS filtered_nodes
                )
            )
            FROM file
            WHERE web_builder.file_id = file.id
            AND web_builder.id = $2
            AND file.user_id = $3",
            &[&all_ids, &builder_id, &user_id],
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete node: {:?}", e);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        if rows == 0 {
            tracing::error!("No rows updated when deleting node");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            ));
        }

        Ok(())
    }
}

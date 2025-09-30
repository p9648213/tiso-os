use crate::{
    models::folder_db::{Folder, FolderType},
    utilities::postgres::DbExecutor,
};

use super::error::AppError;

use axum::http::StatusCode;
use deadpool_postgres::Pool;
use time::OffsetDateTime;
use tokio_postgres::Row;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}

impl User {
    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row.try_get(format!("{prefix}id").as_str()).unwrap_or(None);
        let username: Option<String> = row
            .try_get(format!("{prefix}username").as_str())
            .unwrap_or(None);
        let password: Option<String> = row
            .try_get(format!("{prefix}password").as_str())
            .unwrap_or(None);
        let created_at: Option<OffsetDateTime> = row
            .try_get(format!("{prefix}created_at").as_str())
            .unwrap_or(None);

        Self {
            id,
            username,
            password,
            created_at,
        }
    }

    pub async fn get_user_by_username(
        username: &str,
        columns: Vec<&str>,
        pool: &Pool,
    ) -> Result<Option<Row>, AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let columns = columns.join(",");

        client
            .query_optional(
                &format!(r#"SELECT {columns} FROM "user" WHERE username = $1"#),
                &[&username],
            )
            .await
    }

    pub async fn create_user(username: &str, password: &str, pool: &Pool) -> Result<i32, AppError> {
        let mut client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let txn = client.transaction().await.map_err(|err| {
            tracing::error!("Couldn't start transaction: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let row = txn
            .query_one(
                r#"INSERT INTO "user" (username, password) VALUES ($1, $2) RETURNING id"#,
                &[&username, &password],
            )
            .await?;

        let user = User::try_from(&row, None);

        let user_id = user.id.ok_or_else(|| {
            tracing::error!("No id column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        txn.execute(
            "INSERT INTO folder (user_id, folder_name, folder_type) VALUES 
                ($1, $2, $3), 
                ($1, $4, $5),
                ($1, $6, $7)",
            &[
                &user_id,
                &"Desktop",
                &FolderType::Desktop,
                &"This PC",
                &FolderType::Root,
                &"Taskbar",
                &FolderType::Taskbar,
            ],
        )
        .await?;

        let row = txn
            .query_one(
                "SELECT id FROM folder WHERE user_id = $1 AND folder_type = $2",
                &[&user_id, &FolderType::Taskbar],
            )
            .await?;

        let taskbar_folder = Folder::try_from(&row, None);

        let taskbar_folder_id = taskbar_folder.id.ok_or_else(|| {
            tracing::error!("No id column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        txn.execute(
            "INSERT INTO file (user_id, folder_id, file_name, file_type) VALUES 
                ($1, $2, 'Calculator', 'Calculator'),
                ($1, $2, 'Snake', 'Snake'),
                ($1, $2, 'FlappyBird', 'FlappyBird'),
                ($1, $2, 'Music Player', 'Music')",
            &[&user_id, &taskbar_folder_id],
        )
        .await?;

        txn.execute(
            "INSERT INTO display_setting (user_id) VALUES ($1)",
            &[&user_id],
        )
        .await?;

        let row = txn
            .query_one(
                "SELECT id FROM folder WHERE user_id = $1 AND folder_type = $2",
                &[&user_id, &FolderType::Desktop],
            )
            .await?;

        let desktop_folder = Folder::try_from(&row, None);

        let desktop_folder_id = desktop_folder.id.ok_or_else(|| {
            tracing::error!("No id column or value is null");
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        txn.execute(
            "INSERT INTO file (user_id, folder_id, file_name, file_type) VALUES 
                ($1, $2, 'This PC', 'ThisPC')",
            &[&user_id, &desktop_folder_id],
        )
        .await?;

        txn.commit().await.map_err(|err| {
            tracing::error!("Couldn't commit transaction: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        Ok(user_id)
    }
}

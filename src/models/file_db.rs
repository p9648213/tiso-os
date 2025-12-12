use axum::http::StatusCode;
use deadpool_postgres::Pool;
use postgres_types::{FromSql, ToSql};
use time::OffsetDateTime;
use tokio_postgres::Row;

use crate::{
    models::{
        error::AppError,
        folder_db::{Folder, FolderSortType},
    },
    utilities::postgres::DbExecutor,
};

#[derive(Debug, ToSql, FromSql, Clone, PartialEq, Eq)]
#[postgres(name = "filetype")]
pub enum FileType {
    Txt,
    Calculator,
    Snake,
    FlappyBird,
    ThisPC,
    Music,
    WebBuilder,
    Terminal,
    Resume
}

#[derive(Debug)]
pub struct File {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub folder_id: Option<i32>,
    pub file_name: Option<String>,
    pub file_type: Option<FileType>,
    pub path: Option<String>,
    pub desktop_position: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}

impl File {
    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row.try_get(format!("{prefix}id").as_str()).unwrap_or(None);
        let user_id: Option<i32> = row
            .try_get(format!("{prefix}user_id").as_str())
            .unwrap_or(None);
        let folder_id: Option<i32> = row
            .try_get(format!("{prefix}folder_id").as_str())
            .unwrap_or(None);
        let file_name: Option<String> = row
            .try_get(format!("{prefix}file_name").as_str())
            .unwrap_or(None);
        let file_type: Option<FileType> = row
            .try_get(format!("{prefix}file_type").as_str())
            .unwrap_or(None);
        let desktop_position: Option<String> = row
            .try_get(format!("{prefix}desktop_position").as_str())
            .unwrap_or(None);
        let path: Option<String> = row
            .try_get(format!("{prefix}path").as_str())
            .unwrap_or(None);
        let created_at: Option<OffsetDateTime> = row
            .try_get(format!("{prefix}created_at").as_str())
            .unwrap_or(None);

        Self {
            id,
            user_id,
            folder_id,
            file_name,
            file_type,
            desktop_position,
            path,
            created_at,
        }
    }

    pub fn try_from_vec(rows: Vec<Row>, prefix: Option<&str>) -> Vec<Self> {
        rows.into_iter()
            .map(|row| Self::try_from(&row, prefix))
            .collect()
    }

    pub async fn get_file(
        id: i32,
        user_id: i32,
        columns: Vec<&str>,
        pool: &Pool,
    ) -> Result<File, AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let columns = columns.join(",");

        let row = client
            .query_one(
                &format!("SELECT {columns} FROM file WHERE id = $1 AND user_id = $2"),
                &[&id, &user_id],
            )
            .await?;

        Ok(Self::try_from(&row, None))
    }

    pub async fn get_taskbar_menu_files(
        user_id: i32,
        columns: Vec<&str>,
        pool: &Pool,
    ) -> Result<Vec<File>, AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let taskbar_folder = Folder::get_taskbar_folder(user_id, vec!["id"], pool).await?;

        let columns = columns.join(",");

        let rows = client
            .query(
                &format!("SELECT {columns} FROM file WHERE user_id = $1 AND folder_id = $2"),
                &[&user_id, &taskbar_folder.id.unwrap()],
            )
            .await?;

        Ok(Self::try_from_vec(rows, None))
    }

    pub async fn create_file(
        user_id: i32,
        folder_id: i32,
        mut file_name: String,
        file_type: FileType,
        desktop_position: Option<String>,
        mut path: String,
        pool: &Pool,
    ) -> Result<File, AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let initial_file_name = file_name.clone();

        let sql = "
                SELECT * FROM (
                    SELECT id from folder WHERE parent_folder_id = $1 AND folder_name = $2
                    UNION
                    SELECT id from file WHERE folder_id = $1 AND file_name = $2
                ) AS combined
            ";

        let mut row = client.query(sql, &[&folder_id, &file_name]).await?;

        while !row.is_empty() {
            let random_numb = rand::random_range(0..1000);
            file_name = format!("{} {}", initial_file_name, random_numb);
            println!("{}", file_name);
            row = client.query(sql, &[&folder_id, &file_name]).await?;
        }

        path = format!("{}/{}", path, file_name);

        let row = client
            .query_one(
                "INSERT INTO file (user_id, folder_id, file_name, file_type, desktop_position, path) 
                    VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, file_name",
                &[
                    &user_id,
                    &folder_id,
                    &file_name,
                    &file_type,
                    &desktop_position,
                    &path
                ],
            )
            .await?;

        Ok(Self::try_from(&row, None))
    }

    pub async fn update_desktop_position(
        id: i32,
        desktop_id: i32,
        user_id: i32,
        desktop_position: Option<String>,
        current_sort_type: &Option<FolderSortType>,
        pool: &Pool,
    ) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let rows = client
            .execute(
                "UPDATE file SET desktop_position = $1 WHERE id = $2 AND user_id = $3",
                &[&desktop_position, &id, &user_id],
            )
            .await?;

        if rows == 0 {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error updating file desktop position",
            ));
        }

        let should_update_sort_type = current_sort_type
            .as_ref()
            .map(|t| *t != FolderSortType::Custom)
            .unwrap_or(true);

        if should_update_sort_type {
            let rows = client
                .execute(
                    "UPDATE folder SET sort_type = $1 WHERE id = $2",
                    &[&FolderSortType::Custom, &desktop_id],
                )
                .await?;

            if rows == 0 {
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error updating folder sort type",
                ));
            }
        }

        Ok(())
    }

    pub async fn delete_file(id: i32, user_id: i32, pool: &Pool) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let rows = client
            .execute(
                "DELETE FROM file WHERE id = $1 AND user_id = $2",
                &[&id, &user_id],
            )
            .await?;

        if rows == 0 {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error deleting file",
            ));
        }

        Ok(())
    }

    pub async fn rename_file(
        id: i32,
        user_id: i32,
        file_name: &str,
        pool: &Pool,
    ) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Couldn't get postgres client: {:?}", error),
            )
        })?;

        let row = client
            .execute(
                "UPDATE file SET file_name = $1 WHERE id = $2 AND user_id = $3",
                &[&file_name, &id, &user_id],
            )
            .await?;

        if row == 0 {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error updating file name",
            ));
        }

        Ok(())
    }
}

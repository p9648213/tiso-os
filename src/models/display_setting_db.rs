use std::fmt;

use axum::http::StatusCode;
use deadpool_postgres::Pool;
use postgres_types::{FromSql, ToSql};
use time::OffsetDateTime;
use tokio_postgres::Row;

use crate::{models::error::AppError, utilities::postgres::DbExecutor};

#[derive(Debug, ToSql, FromSql, Clone, PartialEq, Eq)]
#[postgres(name = "backgroundtype")]
pub enum BackgroundType {
    SolidColor,
    Picture,
}

impl fmt::Display for BackgroundType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BackgroundType::SolidColor => write!(f, "SolidColor"),
            BackgroundType::Picture => write!(f, "Picture"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisplaySetting {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub background_type: Option<BackgroundType>,
    pub background_picture: Option<Vec<u8>>,
    pub background_color: Option<String>,
    pub background_content_type: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}

impl DisplaySetting {
    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row.try_get(format!("{prefix}id").as_str()).unwrap_or(None);
        let user_id: Option<i32> = row
            .try_get(format!("{prefix}user_id").as_str())
            .unwrap_or(None);
        let background_type: Option<BackgroundType> = row
            .try_get(format!("{prefix}background_type").as_str())
            .unwrap_or(None);
        let background_picture: Option<Vec<u8>> = row
            .try_get(format!("{prefix}background_picture").as_str())
            .unwrap_or(None);
        let background_color: Option<String> = row
            .try_get(format!("{prefix}background_color").as_str())
            .unwrap_or(None);
        let background_content_type: Option<String> = row
            .try_get(format!("{prefix}background_content_type").as_str())
            .unwrap_or(None);
        let created_at: Option<OffsetDateTime> = row
            .try_get(format!("{prefix}created_at").as_str())
            .unwrap_or(None);

        Self {
            id,
            user_id,
            background_type,
            background_picture,
            background_color,
            background_content_type,
            created_at,
        }
    }

    pub async fn get_setting_by_user_id(
        user_id: i32,
        columns: Vec<&str>,
        pool: &Pool,
    ) -> Result<DisplaySetting, AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let columns = columns.join(",");

        let row = client
            .query_one(
                &format!(r#"SELECT {columns} FROM "display_setting" WHERE user_id = $1"#),
                &[&user_id],
            )
            .await?;

        Ok(Self::try_from(&row, None))
    }

    pub async fn update_background_picture_by_user_id(
        user_id: i32,
        background: Option<Vec<u8>>,
        content_type: &str,
        pool: &Pool,
    ) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let row = client
            .execute(
                "UPDATE display_setting SET background_picture = $1, background_content_type = $2 WHERE user_id = $3",
                &[&background, &content_type, &user_id],
            )
            .await?;

        if row == 0 {
            tracing::error!("Error updating background");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            ));
        }

        Ok(())
    }

    pub async fn update_background_type_by_user_id(
        user_id: i32,
        background_type: &BackgroundType,
        pool: &Pool,
    ) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let row = client
            .execute(
                "UPDATE display_setting SET background_type = $1 WHERE user_id = $2",
                &[&background_type, &user_id],
            )
            .await?;

        if row == 0 {
            tracing::error!("Error updating background type");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            ));
        }

        Ok(())
    }

    pub async fn update_background_color_by_user_id(
        user_id: i32,
        background_color: String,
        pool: &Pool,
    ) -> Result<(), AppError> {
        let client = pool.get().await.map_err(|error| {
            tracing::error!("Couldn't get postgres client: {:?}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
        })?;

        let row = client
            .execute(
                "UPDATE display_setting SET background_color = $1 WHERE user_id = $2",
                &[&background_color, &user_id],
            )
            .await?;

        if row == 0 {
            tracing::error!("Error updating background color");
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server Error",
            ));
        }

        Ok(())
    }
}

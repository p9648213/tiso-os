use crate::utilities::postgres::{query_one, query_optional};

use super::error::AppError;

use deadpool_postgres::Pool;
use time::OffsetDateTime;
use tokio_postgres::Row;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<OffsetDateTime>
}

impl User {
    pub fn try_from(row: &Row, prefix: Option<&str>) -> Self {
        let prefix = prefix.unwrap_or("");

        let id: Option<i32> = row
            .try_get(format!("{}id", prefix).as_str())
            .unwrap_or(None);
        let username: Option<String> = row
            .try_get(format!("{}username", prefix).as_str())
            .unwrap_or(None);
        let password: Option<String> = row
            .try_get(format!("{}password", prefix).as_str())
            .unwrap_or(None);
        let created_at: Option<OffsetDateTime> = row
            .try_get(format!("{}created_at", prefix).as_str())
            .unwrap_or(None);

        Self {
            id,
            username,
            password,
            created_at
        }
    }

    pub async fn get_user_by_username(
        username: &str,
        pool: &Pool,
        columns: Vec<&str>,
    ) -> Result<Option<Row>, AppError> {
        let columns = columns.join(",");

        query_optional(
            &format!("SELECT {} FROM users WHERE username = $1", columns),
            &[&username],
            pool,
        )
        .await
    }

    pub async fn create_user(username: &str, password: &str, pool: &Pool) -> Result<Row, AppError> {
        query_one(
            "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id",
            &[&username, &password],
            pool,
        )
        .await
    }
}

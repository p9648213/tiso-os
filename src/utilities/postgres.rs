use axum::http::StatusCode;
use deadpool_postgres::{GenericClient, Manager, ManagerConfig, Pool, RecyclingMethod};
use postgres_types::ToSql;
use tokio_postgres::{NoTls, Row};

use crate::{
    contanst::{
        PG_DBNAME, PG_HOST, PG_PASSWORD, PG_PORT, PG_SOCKET_DIR, PG_USER, POSTGRE_UNIX_SOCKET,
    },
    models::error::AppError,
};

#[async_trait::async_trait]
pub trait DbExecutor {
    async fn query(
        &self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, AppError>;
    async fn query_one(&self, query: &str, params: &[&(dyn ToSql + Sync)])
    -> Result<Row, AppError>;
    async fn query_optional(
        &self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, AppError>;
    async fn execute(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, AppError>;
}

#[async_trait::async_trait]
impl<T> DbExecutor for T
where
    T: GenericClient + Sync,
{
    async fn query(
        &self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, AppError> {
        let stmt = self.prepare(query).await.map_err(|e| {
            tracing::error!("Prepare Error: {:?}", e);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;
        self.query(&stmt, params).await.map_err(|e| {
            tracing::error!("query error: {:?}", e);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })
    }

    async fn query_one(
        &self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, AppError> {
        let stmt = self.prepare(query).await.map_err(|e| {
            tracing::error!("Prepare Error: {:?}", e);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;
        self.query_one(&stmt, params).await.map_err(|e| {
            tracing::error!("query_one error: {:?}", e);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })
    }

    async fn query_optional(
        &self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, AppError> {
        let stmt = self.prepare(query).await.map_err(|e| {
            tracing::error!("Prepare Error: {:?}", e);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;
        self.query_opt(&stmt, params).await.map_err(|e| {
            tracing::error!("query_opt error: {:?}", e);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })
    }

    async fn execute(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, AppError> {
        let stmt = self.prepare(query).await.map_err(|e| {
            tracing::error!("Prepare Error: {:?}", e);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;
        self.execute(&stmt, params).await.map_err(|e| {
            tracing::error!("Execute Error: {:?}", e);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })
    }
}

fn create_config() -> tokio_postgres::Config {
    let mut cfg = tokio_postgres::Config::new();
    if POSTGRE_UNIX_SOCKET {
        cfg.host(PG_SOCKET_DIR);
    } else {
        cfg.host(PG_HOST);
        cfg.port(PG_PORT);
    }
    cfg.dbname(PG_DBNAME);
    cfg.user(PG_USER);
    cfg.password(PG_PASSWORD);
    cfg
}

pub fn create_pool() -> Pool {
    let pg_config = create_config();

    let manager_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let manager = Manager::from_config(pg_config, NoTls, manager_config);

    Pool::builder(manager).max_size(16).build().unwrap()
}

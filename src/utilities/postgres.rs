use axum::http::StatusCode;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use postgres_types::ToSql;
use tokio_postgres::{NoTls, Row};

use crate::{
    contanst::{PG_DBNAME, PG_PASSWORD, PG_SOCKET_DIR, PG_USER},
    models::error::AppError,
};

fn create_config() -> tokio_postgres::Config {
    let mut cfg = tokio_postgres::Config::new();
    cfg.host(PG_SOCKET_DIR);
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

pub async fn query(
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    pool: &Pool,
) -> Result<Vec<Row>, AppError> {
    let client = pool.get().await.map_err(|error| {
        tracing::error!("Couldn't get postgres client: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let stmt = client.prepare(query).await.map_err(|error| {
        tracing::error!("Couldn't prepare statement: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let row = client.query(&stmt, params).await.map_err(|error| {
        tracing::error!("Couldn't query statement: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    Ok(row)
}

pub async fn query_optional(
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    pool: &Pool,
) -> Result<Option<Row>, AppError> {
    let client = pool.get().await.map_err(|error| {
        tracing::error!("Couldn't get postgres client: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let stmt = client.prepare(query).await.map_err(|error| {
        tracing::error!("Couldn't prepare statement: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let row = client.query_opt(&stmt, params).await.map_err(|error| {
        tracing::error!("Couldn't query statement: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    Ok(row)
}

pub async fn query_one(
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    pool: &Pool,
) -> Result<Row, AppError> {
    let client = pool.get().await.map_err(|error| {
        tracing::error!("Couldn't get postgres client: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let stmt = client.prepare(query).await.map_err(|error| {
        tracing::error!("Couldn't prepare statement: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let row = client.query_one(&stmt, params).await.map_err(|error| {
        tracing::error!("Couldn't query statement: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    Ok(row)
}

pub async fn excute(
    query: &str,
    params: &[&(dyn ToSql + Sync)],
    pool: &Pool,
) -> Result<u64, AppError> {
    let client = pool.get().await.map_err(|error| {
        tracing::error!("Couldn't get postgres client: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let stmt = client.prepare(query).await.map_err(|error| {
        tracing::error!("Couldn't prepare statement: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    let row = client.execute(&stmt, params).await.map_err(|error| {
        tracing::error!("Couldn't execute statement: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server error")
    })?;

    Ok(row)
}

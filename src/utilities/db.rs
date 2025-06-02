use crate::models::error::AppError;
use axum::http::StatusCode;
use deadpool_postgres::Pool;
use tokio_postgres::{Row, types::ToSql};

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

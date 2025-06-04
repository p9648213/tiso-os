use crate::{contanst::ALLOW_ORIGIN, models::error::AppError};
use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};

pub async fn csrf_middleware(request: Request, next: Next) -> Result<impl IntoResponse, AppError> {
    let csrf_header = request.headers().get("X-Csrf-Protection");

    if csrf_header.is_none() {
        tracing::error!("X-Csrf-Protection header is missing");
        return Err(AppError::new(StatusCode::FORBIDDEN, "Forbidden"));
    }

    let origin = request.headers().get("Origin");

    if let Some(origin) = origin {
        let origin = origin.to_str().map_err(|error| {
            tracing::error!("Failed to get origin header: {}", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Error")
        })?;

        if origin != ALLOW_ORIGIN {
            tracing::error!("Origin is not allowed");
            return Err(AppError::new(StatusCode::FORBIDDEN, "Forbidden"));
        }
    } else {
        tracing::error!("Origin header is missing");
        return Err(AppError::new(StatusCode::FORBIDDEN, "Forbidden"));
    }

    Ok(next.run(request).await)
}

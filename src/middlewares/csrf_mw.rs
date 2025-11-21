use crate::{constant::ALLOW_ORIGIN, models::error::AppError};
use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};

pub async fn csrf_middleware(request: Request, next: Next) -> Result<impl IntoResponse, AppError> {
    let csrf_header = request.headers().get("X-Csrf-Protection");

    if csrf_header.is_none() {
        return Err(AppError::new(StatusCode::FORBIDDEN, "X-Csrf-Protection header is missing"));
    }

    let origin = request.headers().get("Origin");

    if let Some(origin) = origin {
        let origin = origin.to_str().map_err(|error| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, &format!("Failed to get origin header: {}", error))
        })?;

        if origin != ALLOW_ORIGIN {
            return Err(AppError::new(StatusCode::FORBIDDEN, &format!("Origin is not allowed: {}", origin)));
        }
    } else {
        return Err(AppError::new(StatusCode::FORBIDDEN, "Origin header is missing"));
    }

    Ok(next.run(request).await)
}

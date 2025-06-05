use crate::models::{error::AppError, state::SessionMap};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};

pub async fn session_middleware(
    State(session): State<SessionMap>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    session.pin().get("1");
    // session.pin().insert("1".to_string(), "".to_string());

    Ok(next.run(request).await)
}

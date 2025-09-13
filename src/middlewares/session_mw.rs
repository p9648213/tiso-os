use crate::models::{error::AppError, state::SessionMap};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

#[derive(Clone)]
pub struct UserId(pub Option<String>);

pub async fn session_middleware(
    State(session_map): State<SessionMap>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let session_map = session_map.pin_owned();

    let jar = CookieJar::from_headers(request.headers());

    let session = jar
        .get("session")
        .map(|cookie| cookie.value())
        .unwrap_or_default();

    let user_id = session_map.get(session);

    match user_id {
        Some(user_id) => request
            .extensions_mut()
            .insert(UserId(Some(user_id.to_owned()))),
        None => request
            .extensions_mut()
            // .insert(UserId(None)),
            .insert(UserId(Some(String::from("6")))), // For fast test
    };

    Ok(next.run(request).await)
}

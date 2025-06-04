use axum::{extract::Request, middleware::Next, response::IntoResponse};
use tower_sessions::Session;

use crate::models::error::AppError;

#[derive(Clone, Debug)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub image_url: String,
    pub rs_role: String,
}

#[derive(Clone, Debug)]
pub struct UserAuth(pub Option<UserInfo>);

pub async fn auth_middleware(
    session: Session,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let user_id = session.get("user-id").await.unwrap_or_default();

    if let Some(id) = user_id {
        let username = session.get("user-name").await.unwrap_or_default();
        let image_url = session.get("user-image").await.unwrap_or_default();
        let rs_role = session.get("user-rs-role").await.unwrap_or_default();

        let user_info = UserInfo {
            id,
            username: username.unwrap_or_default(),
            image_url: image_url.unwrap_or_default(),
            rs_role: rs_role.unwrap_or_default(),
        };

        request.extensions_mut().insert(UserAuth(Some(user_info)));

        Ok(next.run(request).await.into_response())
    } else {
        request.extensions_mut().insert(UserAuth(None));
        Ok(next.run(request).await.into_response())
    }
}

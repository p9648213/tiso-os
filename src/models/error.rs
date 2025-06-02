use axum::{http::StatusCode, response::IntoResponse};
pub struct AppError(StatusCode, String);

impl AppError {
    pub fn new(status_code: StatusCode, message: &str) -> Self {
        Self(status_code, message.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}

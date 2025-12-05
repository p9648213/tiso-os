use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
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

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

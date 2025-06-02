use axum::{extract::Request, middleware::Next, response::Response};

pub async fn request_log(req: Request, next: Next) -> Response {
  tracing::info!(
      "Request: method {} path {}",
      req.method(),
      req.uri()
  );
  next.run(req).await
}
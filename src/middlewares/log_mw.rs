use axum::{extract::Request, middleware::Next, response::Response, body::Body};
use tracing::Span;

pub async fn request_log(req: Request, next: Next) -> Response {
  tracing::info!(
      "Request: method {} path {}",
      req.method(),
      req.uri()
  );
  next.run(req).await
}

pub fn response_log(response: &Response<Body>, latency: std::time::Duration, _: &Span) {
    tracing::info!("<- Response: status {} in {:?}", response.status(), latency)
}

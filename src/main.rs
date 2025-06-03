use tiso_os::{contanst::PORT, router::create_router, utilities::tracing::init_tracing};

#[tokio::main]
async fn main() {
    init_tracing();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();

    let app = create_router();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.await).await.unwrap();
}

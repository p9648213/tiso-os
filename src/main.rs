use tiso_os::{
    constant::PORT,
    router::create_router,
    utilities::{migration::init_database, postgres::create_pool, tracing::init_tracing},
};

#[tokio::main]
async fn main() {
    init_tracing();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}"))
        .await
        .unwrap();

    let pool = create_pool();

    init_database(&pool).await;

    let app = create_router(pool);

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.await).await.unwrap();
}

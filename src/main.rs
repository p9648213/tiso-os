use clap::Parser;
use tiso_os::{
    router::create_router,
    utilities::{config::EnvConfig, postgres, tracing::init_tracing},
};

#[tokio::main]
async fn main() {
    init_tracing();

    dotenvy::dotenv().ok();

    let config = EnvConfig::parse();

    let pg_pool = postgres::create_pool(&config);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &config.port))
        .await
        .unwrap();

    let app = create_router(pg_pool, config);

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app.await).await.unwrap();
}

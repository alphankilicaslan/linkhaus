mod models;
mod routes;
mod state;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt};

use crate::routes::build_router;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("linkhaus_backend=info,tower_http=info"));

    fmt().with_env_filter(filter).init();

    let state = AppState::new_shared();
    let app = build_router(state);

    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("LinkHaus Core listening on http://{}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("Shutdown signal received, stopping LinkHaus Core");
}

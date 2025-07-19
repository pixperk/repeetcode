use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::routes::create_router;

mod error;
mod routes;
mod gql;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("repeetcode=debug,tower_http=debug")),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .expect("Failed to initialize tracing subscriber");

    let port = 3000;
    let addr = format!("0.0.0.0:{port}");

    tracing::info!("Starting repeetcode server on {}", addr);

    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(err) => {
            tracing::error!("Failed to bind to {}: {}", addr, err);
            return Err(err.into());
        }
    };

    let app = create_router();

    if let Err(err) = axum::serve(listener, app.into_make_service()).await {
        tracing::error!("Server error: {}", err);
        return Err(err.into());
    }

    Ok(())
}

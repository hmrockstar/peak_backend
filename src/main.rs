use axum::serve;
use tokio::{
    net::TcpListener,
    signal::unix::{signal, SignalKind},
    sync::oneshot,
};
use tracing::info;

mod app;
mod config;
mod handlers;
mod services;

#[tokio::main]
async fn main() {
    config::setup_logging();
    let config = config::Config::new();

    let app = app::create_app();
    let listener = TcpListener::bind(config.addr).await.unwrap();

    // Handle multiple termination signals
    let (tx, rx) = oneshot::channel::<()>();
    tokio::spawn(async move {
        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        let mut sigint = signal(SignalKind::interrupt()).unwrap();

        tokio::select! {
            _ = sigterm.recv() => info!("Received SIGTERM"),
            _ = sigint.recv() => info!("Received SIGINT"),
        }
        tx.send(()).unwrap();
    });

    info!("Starting server on {}", config.addr);
    serve(listener, app)
        .with_graceful_shutdown(async {
            rx.await.ok();
            println!("Shutting down gracefully...");
        })
        .await
        .unwrap();
}

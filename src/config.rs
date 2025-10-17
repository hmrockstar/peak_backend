use std::net::SocketAddr;
use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter};

pub struct Config {
    pub addr: SocketAddr,
}

impl Config {
    pub fn new() -> Self {
        let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
        Self { addr }
    }
}

pub fn setup_logging() {
    std::env::set_var(
        "PROD",
        std::env::var("PROD").unwrap_or_else(|_| "info".into()),
    );

    registry()
        .with(EnvFilter::from_env("PROD"))
        .with(fmt::layer().json())
        .init();
}

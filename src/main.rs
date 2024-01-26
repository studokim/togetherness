use clap::Parser;
use std::net::SocketAddr;

mod config;
mod layers;
mod log;
mod model;
mod rest;
mod router;
mod state;
mod static_server;
mod timer;

use crate::config::Args;

#[tokio::main]
async fn main() {
    log::configure(tracing::Level::DEBUG, true);

    let args = Args::parse();
    if args.database_url == None {
        log::warn!("DATABASE_URL is not set, limited functionality")
    }
    let addr = SocketAddr::from((args.ip, args.port));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Cannot bind the requested address");
    log::info!("Listening on {} ...", addr);

    let router = router::new();
    axum::serve(listener, router)
        .await
        .expect("Cannot start the axum server");
}

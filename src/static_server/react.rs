use axum::Router;
use clap::Parser;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

use crate::log;
use crate::Args;

pub fn assets() -> Router {
    let dir = Args::parse().front_dir;
    log::info!(
        "Serving assets from {}",
        dir.canonicalize()
            .expect("Path to assets must exist")
            .display()
    );
    Router::new().nest_service(
        "/",
        ServeDir::new(&dir).fallback(ServeFile::new(&dir.join("index.html"))),
    )
}

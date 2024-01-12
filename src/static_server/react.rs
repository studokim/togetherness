use clap::Parser;
use tower_http::services::ServeDir;

use crate::log;
use crate::Args;

pub fn assets() -> ServeDir {
    let dir = Args::parse().front_dir;
    log::debug!(
        "Serving assets from {}",
        dir.canonicalize()
            .expect("Path to assets must exist")
            .display()
    );
    ServeDir::new(dir)

    // axum::serve(listener, app.layer(TraceLayer::new_for_http()))
    // .await
    // .unwrap();
}

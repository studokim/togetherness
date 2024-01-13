use clap::Parser;
use tower_http::services::ServeDir;

use crate::log;
use crate::Args;

pub fn assets() -> ServeDir {
    let dir = Args::parse().front_dir;
    log::info!(
        "Serving assets from {}",
        dir.canonicalize()
            .expect("Path to assets must exist")
            .display()
    );
    ServeDir::new(dir)
}

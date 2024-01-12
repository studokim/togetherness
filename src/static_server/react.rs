use std::path::Path;
use tower_http::services::ServeDir;

use crate::log;

pub fn assets() -> ServeDir {
    let dir = Path::new("react");
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

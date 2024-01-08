pub use tracing::{debug, error, info, warn};

use time::UtcOffset;
use tracing_subscriber::{fmt::time::OffsetTime, FmtSubscriber};

pub fn configure() {
    let offset = UtcOffset::from_hms(3, 0, 0).expect("Should get MSK offset");
    let format = time::format_description::well_known::Iso8601::DATE_TIME;
    let timer = OffsetTime::new(offset, format);

    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::TRACE)
        .with_thread_names(true)
        .with_timer(timer)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
}

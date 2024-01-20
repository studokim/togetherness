pub use tracing::{debug, error, info, warn};

use time::UtcOffset;
use tracing_subscriber::{fmt::time::OffsetTime, FmtSubscriber};

pub fn configure(level: tracing::Level) {
    let offset = UtcOffset::from_hms(3, 0, 0).expect("Should get MSK offset");
    let format = time::format_description::well_known::Iso8601::DATE_TIME;
    let timer = OffsetTime::new(offset, format);

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_thread_names(true)
        .with_timer(timer)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");
}

use chrono::{DateTime, Utc};
use shadow_rs::shadow;

shadow!(build);

pub const BUILD_TIME: DateTime<Utc> =
    DateTime::from_timestamp(build::BUILD_TIMESTAMP, 0).expect("invalid build timestamp");

pub const COMMIT: &str = build::SHORT_COMMIT;

pub const RUSTC: &str = build::RUST_VERSION;

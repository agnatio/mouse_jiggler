use serde::{Deserialize, Serialize};

// Default constants
pub const JIGGLE_INTERVAL_SECS: u64 = 5;
pub const JIGGLE_DELAY_MS: u64 = 500;
pub const JIGGLE_DISTANCE: i32 = 1;

// User settings (persisted)
#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub interval_secs: u64,
    pub delay_ms: u64,
    pub distance: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            interval_secs: JIGGLE_INTERVAL_SECS,
            delay_ms: JIGGLE_DELAY_MS,
            distance: JIGGLE_DISTANCE,
        }
    }
}

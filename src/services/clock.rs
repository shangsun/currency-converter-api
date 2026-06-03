use chrono::{DateTime, Utc};

/// Source of the current time.
///
/// Injected so time-dependent logic can be driven deterministically in tests
/// instead of reading the wall clock directly.
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

/// Real wall-clock implementation.
#[derive(Clone, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

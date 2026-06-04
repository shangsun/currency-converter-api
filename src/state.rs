use crate::error::ApiError;
use crate::models::DailyRate;
use crate::services::{Clock, Fetcher, Store};
use std::sync::Arc;

/// Shared application state injected into every handler.
///
/// Holds the storage backend, the upstream fetcher, and the clock. The rate
/// read path goes through [`AppState::current_rates`]; `fetcher` and `clock`
/// are the seams a read-through cache builds on.
#[derive(Clone)]
pub struct AppState {
    pub store: Arc<dyn Store>,
    pub fetcher: Arc<dyn Fetcher>,
    pub clock: Arc<dyn Clock>,
}

impl AppState {
    pub fn new(store: Arc<dyn Store>, fetcher: Arc<dyn Fetcher>, clock: Arc<dyn Clock>) -> Self {
        Self {
            store,
            fetcher,
            clock,
        }
    }

    /// The rate read path used by the handlers.
    ///
    /// Currently returns whatever is in the store (push-based: the scheduler
    /// writes rates, reads serve them); it never fetches on demand.
    pub async fn current_rates(&self) -> Result<DailyRate, ApiError> {
        self.store
            .get_rates()
            .await?
            .ok_or(ApiError::NoRatesAvailable)
    }
}

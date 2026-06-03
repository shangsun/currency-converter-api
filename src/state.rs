use crate::services::{Clock, Fetcher, Store};
use std::sync::Arc;

/// Shared application state injected into every handler.
///
/// Holds the storage backend, the upstream fetcher, and the clock. Handlers
/// read rates from `store`; `fetcher` and `clock` are the seams a read-through
/// cache layer builds on.
#[derive(Clone)]
pub struct AppState {
    pub store: Arc<dyn Store>,
    pub fetcher: Arc<dyn Fetcher>,
    pub clock: Arc<dyn Clock>,
}

use crate::error::ApiError;
use crate::models::DailyRate;
use async_trait::async_trait;
use std::sync::{Arc, RwLock};

/// Storage backend for the latest daily exchange rates.
///
/// Abstracts over the concrete backend (Redis in production, in-memory by
/// default / for tests) so the rest of the app depends only on this trait.
#[async_trait]
pub trait Store: Send + Sync {
    /// Retrieve the currently stored rates, if any.
    async fn get_rates(&self) -> Result<Option<DailyRate>, ApiError>;

    /// Persist the given rates as the latest.
    async fn store_rates(&self, rates: &DailyRate) -> Result<(), ApiError>;

    /// Date string of the most recently stored rates, if any.
    async fn get_last_update_date(&self) -> Result<Option<String>, ApiError>;

    /// Backend health probe.
    async fn health_check(&self) -> Result<(), ApiError>;
}

/// In-memory [`Store`] implementation. Default backend — requires no external
/// service, so the app builds and tests run fully offline and deterministically.
#[derive(Clone, Default)]
pub struct InMemoryStore {
    inner: Arc<RwLock<Option<DailyRate>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Store for InMemoryStore {
    async fn get_rates(&self) -> Result<Option<DailyRate>, ApiError> {
        // Guard is dropped within this statement; never held across an await.
        Ok(self.inner.read().unwrap().clone())
    }

    async fn store_rates(&self, rates: &DailyRate) -> Result<(), ApiError> {
        *self.inner.write().unwrap() = Some(rates.clone());
        Ok(())
    }

    async fn get_last_update_date(&self) -> Result<Option<String>, ApiError> {
        Ok(self.inner.read().unwrap().as_ref().map(|r| r.date.clone()))
    }

    async fn health_check(&self) -> Result<(), ApiError> {
        Ok(())
    }
}

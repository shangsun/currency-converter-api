use crate::error::ApiError;
use crate::models::DailyRate;
use async_trait::async_trait;

/// Upstream source of daily exchange rates.
///
/// Abstracts the concrete upstream (ECB over HTTP in production, a fake in
/// tests) so callers can be exercised without real network access.
#[async_trait]
pub trait Fetcher: Send + Sync {
    /// Fetch and parse the latest daily rates from the upstream source.
    async fn fetch_rates(&self) -> Result<DailyRate, ApiError>;
}

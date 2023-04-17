use async_trait::async_trait;

use crate::error::InterestError;
use crate::interest::model::InterestConfiguration;

#[async_trait]
pub trait InterestRepository {
    async fn find_interest_range_by_delayed_days(
        &self,
        delayed_days: i64,
    ) -> Result<InterestConfiguration, InterestError>;
}

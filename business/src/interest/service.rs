use std::sync::Arc;

use crate::error::InterestError;
use crate::interest::model::InterestConfiguration;
use crate::interest::repository::InterestRepository;

pub struct InterestService {
    pub interest_repository: Arc<dyn InterestRepository + Send + Sync>,
}

impl InterestService {
    pub fn new(interest_repository: Arc<dyn InterestRepository + Send + Sync>) -> InterestService {
        Self {
            interest_repository,
        }
    }

    pub async fn find_interest_configuration_range_by_delayed_days(
        &self,
        delayed_days: i64,
    ) -> Result<InterestConfiguration, InterestError> {
        self.interest_repository
            .find_interest_range_by_delayed_days(delayed_days)
            .await
    }
}

use std::sync::Arc;

use crate::bill::model::Bill;
use crate::bill::repository::BillRepository;
use crate::error::{BillError, InterestError};
use crate::interest::service::InterestService;

pub struct BillService {
    pub bill_repository: Arc<dyn BillRepository + Send + Sync>,
    pub interest_service: Arc<InterestService>,
}

impl BillService {
    pub fn new(
        bill_repository: Arc<dyn BillRepository + Send + Sync>,
        interest_service: Arc<InterestService>,
    ) -> BillService {
        Self {
            bill_repository,
            interest_service,
        }
    }

    pub async fn list_bills(&self) -> Result<Vec<Bill>, BillError> {
        self.bill_repository.list_bills().await
    }

    pub async fn create_bill(&self, mut bill: Bill) -> Result<Bill, BillError> {
        if bill.is_delayed() {
            let corrected_amount = self.calculate_corrected_amount(&bill).await?;
            bill.corrected_amount = Some(corrected_amount);
        }

        self.bill_repository.save_bill(&bill).await
    }

    async fn calculate_corrected_amount(&self, bill: &Bill) -> Result<f64, BillError> {
        let delayed_days = bill.calculate_delayed_days();

        let interest_config = self
            .interest_service
            .find_interest_configuration_range_by_delayed_days(delayed_days)
            .await?
            .ok_or(InterestError::NotFound(format!(
                "for {delayed_days} delayed days"
            )))?;

        let original_amount = bill.original_amount;

        let interest = (original_amount * interest_config.interest) * (delayed_days as f64);
        let fine = original_amount * interest_config.fine;

        Ok(original_amount + interest + fine)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Not;

    use chrono::NaiveDate;

    use crate::bill::model::Bill;

    #[test]
    fn should_calculate_delayed_days_correctly() {
        let bill = Bill {
            id: Default::default(),
            description: "".to_string(),
            original_amount: 0.0,
            corrected_amount: None,
            due_date: NaiveDate::from_ymd_opt(2023, 04, 16).unwrap(),
            payment_date: NaiveDate::from_ymd_opt(2023, 04, 20).unwrap(),
        };

        assert!(bill.is_delayed());
        assert_eq!(bill.calculate_delayed_days(), 4);
    }

    #[test]
    fn bill_should_not_be_delayed_when_payment_date_matches_due_date() {
        let bill = Bill {
            id: Default::default(),
            description: "".to_string(),
            original_amount: 0.0,
            corrected_amount: None,
            due_date: NaiveDate::from_ymd_opt(2023, 04, 20).unwrap(),
            payment_date: NaiveDate::from_ymd_opt(2023, 04, 20).unwrap(),
        };

        assert!(bill.is_delayed().not());
    }
}

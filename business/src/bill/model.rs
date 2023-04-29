use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Bill {
    pub id: Uuid,
    pub description: String,
    pub original_amount: BigDecimal,
    pub corrected_amount: Option<BigDecimal>,
    pub due_date: NaiveDate,
    pub payment_date: NaiveDate,
}

impl Bill {
    pub fn is_delayed(&self) -> bool {
        self.due_date < self.payment_date
    }

    pub fn calculate_delayed_days(&self) -> i64 {
        (self.payment_date - self.due_date).num_days()
    }
}

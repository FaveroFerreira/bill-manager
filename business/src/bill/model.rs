use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug)]
pub struct Bill {
    pub id: Uuid,
    pub description: String,
    pub original_amount: f64,
    pub corrected_amount: Option<f64>,
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

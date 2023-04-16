use sqlx::types::chrono::NaiveDate;
use sqlx::types::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct PersistenceBill {
    pub id: Uuid,
    pub description: String,
    pub original_amount: f64,
    pub corrected_amount: Option<f64>,
    pub due_date: NaiveDate,
    pub payment_date: NaiveDate,
}

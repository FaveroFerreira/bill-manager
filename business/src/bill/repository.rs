use async_trait::async_trait;

use crate::bill::model::Bill;
use crate::error::BillError;

#[async_trait]
pub trait BillRepository {
    async fn save_bill(&self, bill: &Bill) -> Result<Bill, BillError>;
    async fn list_bills(&self) -> Result<Vec<Bill>, BillError>;
}

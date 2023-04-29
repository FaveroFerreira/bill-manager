use std::sync::Arc;

use axum::extract::State;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

use business::bill::model::Bill;

use crate::context::ApplicationContext;
use crate::error::ApiError;
use crate::response::ApiResponse;

#[derive(Debug, Serialize)]
pub struct BillDTO {
    pub id: Uuid,
    pub description: String,
    pub original_amount: BigDecimal,
    pub corrected_amount: Option<BigDecimal>,
    pub due_date: NaiveDate,
    pub payment_date: NaiveDate,
}

impl From<Bill> for BillDTO {
    fn from(bill: Bill) -> Self {
        BillDTO {
            id: bill.id,
            description: bill.description,
            original_amount: bill.original_amount.with_scale(2),
            corrected_amount: bill.corrected_amount.map(|it| it.with_scale(2)),
            due_date: bill.due_date,
            payment_date: bill.payment_date,
        }
    }
}

#[axum_macros::debug_handler]
pub async fn handler(
    State(application_context): State<Arc<ApplicationContext>>,
) -> Result<ApiResponse<Vec<BillDTO>>, ApiError> {
    let bills = application_context.bill_service.list_bills().await?;

    let bills_dto = bills
        .into_iter()
        .map(BillDTO::from)
        .collect::<Vec<BillDTO>>();

    Ok(ApiResponse::ok(bills_dto))
}

use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use business::bill::model::Bill;

use crate::context::ApiContext;
use crate::error::ApiError;
use crate::response::ApiResponse;

#[derive(Debug, Deserialize)]
pub struct CreateBillDTO {
    pub description: String,
    pub original_amount: BigDecimal,
    pub due_date: NaiveDate,
    pub payment_date: NaiveDate,
}

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

pub const CREATE_BILL_ENDPOINT: &str = "/bill";

#[axum_macros::debug_handler]
pub async fn create_bill(
    State(api_context): State<Arc<ApiContext>>,
    Json(create_bill_dto): Json<CreateBillDTO>,
) -> Result<ApiResponse<Bill>, ApiError> {
    let bill = Bill {
        id: Uuid::new_v4(),
        description: create_bill_dto.description,
        original_amount: create_bill_dto.original_amount,
        corrected_amount: None,
        due_date: create_bill_dto.due_date,
        payment_date: create_bill_dto.payment_date,
    };

    let created_bill = api_context.bill_service.create_bill(bill).await?;

    Ok(ApiResponse::created(created_bill))
}

pub const LIST_BILLS_ENDPOINT: &str = "/bill";

#[axum_macros::debug_handler]
pub async fn list_bills(
    State(api_context): State<Arc<ApiContext>>,
) -> Result<ApiResponse<Vec<BillDTO>>, ApiError> {
    let bills = api_context.bill_service.list_bills().await?;

    let bills_dto = bills
        .into_iter()
        .map(BillDTO::from)
        .collect::<Vec<BillDTO>>();

    Ok(ApiResponse::ok(bills_dto))
}

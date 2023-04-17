use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use business::bill::model::Bill;

use crate::context::ApplicationContext;

#[derive(Debug, Deserialize)]
pub struct CreateBillDTO {
    pub description: String,
    pub original_amount: BigDecimal,
    pub due_date: NaiveDate,
    pub payment_date: NaiveDate,
}

#[axum_macros::debug_handler]
pub async fn handler(
    State(application_context): State<Arc<ApplicationContext>>,
    Json(create_bill_dto): Json<CreateBillDTO>,
) {
    let bill = Bill {
        id: Uuid::new_v4(),
        description: create_bill_dto.description,
        original_amount: create_bill_dto.original_amount,
        corrected_amount: None,
        due_date: create_bill_dto.due_date,
        payment_date: create_bill_dto.payment_date,
    };

    application_context
        .bill_service
        .create_bill(bill)
        .await
        .unwrap();
}

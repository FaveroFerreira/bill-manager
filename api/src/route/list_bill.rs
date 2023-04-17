use std::sync::Arc;

use axum::extract::State;

use crate::context::ApplicationContext;

#[axum_macros::debug_handler]
pub async fn handler(State(application_context): State<Arc<ApplicationContext>>) {
    let bill = application_context.bill_service.list_bills().await.unwrap();

    println!("{bill:?}");
}

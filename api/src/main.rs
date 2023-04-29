use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;

use infrastructure::environment::load_env;
use infrastructure::telemetry::init_telemetry;
use route::bill_routes;

use crate::context::ApiContext;

mod context;
mod error;
mod response;
mod route;

#[tokio::main]
async fn main() {
    let _guard = init_telemetry();

    let env = load_env();

    let api_context = Arc::new(ApiContext::autowire(&env).await);

    let router = Router::new()
        .route(
            bill_routes::LIST_BILLS_ENDPOINT,
            get(bill_routes::list_bills),
        )
        .route(
            bill_routes::CREATE_BILL_ENDPOINT,
            post(bill_routes::create_bill),
        )
        .with_state(api_context);

    axum::Server::bind(&SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080)))
        .serve(router.into_make_service())
        .await
        .unwrap()
}

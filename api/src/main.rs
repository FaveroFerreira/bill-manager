use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;

use infrastructure::environment::load_env;
use infrastructure::observability::init_tracing;

use crate::context::ApplicationContext;

mod context;
mod route;

#[tokio::main]
async fn main() {
    let _guard = init_tracing();

    let env = load_env();

    let application_context = Arc::new(ApplicationContext::autowire(&env).await);

    let router = Router::new()
        .route("/bill", get(route::list_bill::handler))
        .route("/bill", post(route::create_bill::handler))
        .with_state(application_context);

    axum::Server::bind(&SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080)))
        .serve(router.into_make_service())
        .await
        .unwrap()
}

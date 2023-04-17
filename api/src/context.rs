use std::sync::Arc;

use business::bill::service::BillService;
use business::interest::service::InterestService;
use infrastructure::database::init_db;
use infrastructure::environment::Env;
use persistence::bill::BillPostgresRepository;
use persistence::interest::InterestPostgresRepository;

pub struct ApplicationContext {
    pub bill_service: Arc<BillService>,
    pub interest_service: Arc<InterestService>,
}

impl ApplicationContext {
    pub async fn autowire(env: &Env) -> Self {
        let db_pool = init_db(env).await;

        let interest_repository = Arc::new(InterestPostgresRepository::new(db_pool.clone()));
        let bill_repository = Arc::new(BillPostgresRepository::new(db_pool));

        let interest_service = Arc::new(InterestService::new(interest_repository));
        let bill_service = Arc::new(BillService::new(bill_repository, interest_service.clone()));

        Self {
            bill_service,
            interest_service,
        }
    }
}

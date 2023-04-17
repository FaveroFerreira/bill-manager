use std::path::Path;
use std::time::Duration;

use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::environment::Env;

pub const MIGRATIONS_PATH: &str = "./migrations";

pub async fn init_db(env: &Env) -> PgPool {
    let db_url = format!("{}?application_name=bill-manager", env.database_url);

    let pool = PgPoolOptions::new()
        .min_connections(3)
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&db_url)
        .await
        .unwrap();

    run_migrations(&pool).await;

    pool
}

pub async fn run_migrations(pool: &PgPool) {
    Migrator::new(Path::new(MIGRATIONS_PATH))
        .await
        .unwrap()
        .run(pool)
        .await
        .unwrap()
}

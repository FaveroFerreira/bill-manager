use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use business::error::InterestError;
use business::interest::model::InterestConfiguration;
use business::interest::repository::InterestRepository;

#[derive(Debug, sqlx::FromRow)]
pub struct PersistenceInterestConfiguration {
    pub id: Uuid,
    pub start_range: i64,
    pub end_range: i64,
    pub fine: f64,
    pub interest: f64,
}

impl From<InterestConfiguration> for PersistenceInterestConfiguration {
    fn from(p: InterestConfiguration) -> Self {
        PersistenceInterestConfiguration {
            id: p.id,
            start_range: p.start_range,
            end_range: p.end_range,
            fine: p.fine,
            interest: p.interest,
        }
    }
}

impl From<PersistenceInterestConfiguration> for InterestConfiguration {
    fn from(p: PersistenceInterestConfiguration) -> Self {
        InterestConfiguration {
            id: p.id,
            start_range: p.start_range,
            end_range: p.end_range,
            fine: p.fine,
            interest: p.interest,
        }
    }
}

pub struct InterestPostgresRepository {
    pub pool: PgPool,
}

impl InterestPostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InterestRepository for InterestPostgresRepository {
    async fn find_interest_range_by_delayed_days(
        &self,
        delayed_days: i64,
    ) -> Result<Option<InterestConfiguration>, InterestError> {
        const QUERY: &str =
            "SELECT * FROM interest_configuration WHERE $1 >= start_range AND $1 <= end_range";

        sqlx::query_as::<_, PersistenceInterestConfiguration>(QUERY)
            .bind(delayed_days)
            .fetch_optional(&self.pool)
            .await
            .map(|it| it.map(InterestConfiguration::from))
            .map_err(|e| InterestError::Persistence(format!("{e:?}")))
    }
}

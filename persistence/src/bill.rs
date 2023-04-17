use async_trait::async_trait;
use bigdecimal::BigDecimal;
use sqlx::types::chrono::NaiveDate;
use sqlx::types::Uuid;
use sqlx::PgPool;

use business::bill::model::Bill;
use business::bill::repository::BillRepository;
use business::error::BillError;

#[derive(Debug, sqlx::FromRow)]
pub struct PersistenceBill {
    pub id: Uuid,
    pub description: String,
    pub original_amount: BigDecimal,
    pub corrected_amount: Option<BigDecimal>,
    pub due_date: NaiveDate,
    pub payment_date: NaiveDate,
}

impl From<Bill> for PersistenceBill {
    fn from(bill: Bill) -> Self {
        PersistenceBill {
            id: bill.id,
            description: bill.description,
            original_amount: bill.original_amount,
            corrected_amount: bill.corrected_amount,
            due_date: bill.due_date,
            payment_date: bill.payment_date,
        }
    }
}

impl From<PersistenceBill> for Bill {
    fn from(persistence_bill: PersistenceBill) -> Self {
        Bill {
            id: persistence_bill.id,
            description: persistence_bill.description,
            original_amount: persistence_bill.original_amount,
            corrected_amount: persistence_bill.corrected_amount,
            due_date: persistence_bill.due_date,
            payment_date: persistence_bill.payment_date,
        }
    }
}

pub struct BillPostgresRepository {
    pool: PgPool,
}

impl BillPostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BillRepository for BillPostgresRepository {
    async fn save_bill(&self, bill: &Bill) -> Result<Bill, BillError> {
        const QUERY: &str = r#"
            INSERT INTO bill (id, description, original_amount, corrected_amount, due_date, payment_date) 
            VALUES ($1, $2, $3, $4, $5, $6) 
            RETURNING id, description, original_amount, corrected_amount, due_date, payment_date
        "#;

        sqlx::query_as::<_, PersistenceBill>(QUERY)
            .bind(bill.id)
            .bind(&bill.description)
            .bind(&bill.original_amount)
            .bind(&bill.corrected_amount)
            .bind(bill.due_date)
            .bind(bill.payment_date)
            .fetch_one(&self.pool)
            .await
            .map(Bill::from)
            .map_err(|e| BillError::Persistence(format!("{e:?}")))
    }

    async fn list_bills(&self) -> Result<Vec<Bill>, BillError> {
        const QUERY: &str = "SELECT * FROM bill";

        sqlx::query_as::<_, PersistenceBill>(QUERY)
            .fetch_all(&self.pool)
            .await
            .map(|it| it.into_iter().map(Bill::from).collect())
            .map_err(|e| BillError::Persistence(format!("{e:?}")))
    }
}

use bigdecimal::BigDecimal;
use uuid::Uuid;

#[derive(Debug)]
pub struct InterestConfiguration {
    pub id: Uuid,
    pub start_range: i32,
    pub end_range: i32,
    pub fine: BigDecimal,
    pub interest: BigDecimal,
}

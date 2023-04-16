use uuid::Uuid;

#[derive(Debug)]
pub struct InterestConfiguration {
    pub id: Uuid,
    pub start_range: i64,
    pub end_range: i64,
    pub fine: f64,
    pub interest: f64,
}

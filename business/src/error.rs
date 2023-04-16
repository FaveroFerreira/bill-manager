#[derive(Debug, thiserror::Error)]
pub enum BillError {
    #[error("bill not found {0}")]
    NotFound(String),

    #[error("{0}")]
    Persistence(String),

    #[error("{0}")]
    ParseError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum InterestError {
    #[error("interest not found {0}")]
    NotFound(String),

    #[error("{0}")]
    Persistence(String),

    #[error("{0}")]
    ParseError(String),
}

impl From<InterestError> for BillError {
    fn from(interest_error: InterestError) -> Self {
        match interest_error {
            InterestError::NotFound(e) => BillError::NotFound(e),
            InterestError::Persistence(e) => BillError::Persistence(e),
            InterestError::ParseError(e) => BillError::ParseError(e),
        }
    }
}

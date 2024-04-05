use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CsesError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Parse error @ {0}")]
    ParseError(String),
}
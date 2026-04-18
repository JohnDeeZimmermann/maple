use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid argument provided: {0}")]
    IllegalArgumentError(String),
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid argument provided: {0}")]
    IllegalArgumentError(String),
    #[error("expected token {expected:?}, but found {found:?}")]
    SyntaxError { expected: String, found: String },
    #[error("expected number but found {found:?}")]
    NumberExpectedError { found: String },
    #[error("expected instruction but found {found:?}")]
    InstructionExpectedError { found: String },
    #[error("expected pre-assembler instruction but found {found:?}")]
    PreAssemblerInstructionExpectedError { found: String },
    #[error("expected identifier but found {found:?}")]
    IdentifierExpectedError { found: String },
    #[error("expected string but found {found:?}")]
    StringExpectedError { found: String },
}

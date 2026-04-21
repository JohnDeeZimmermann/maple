use crate::assembler::parser::errors::parse_errors::ParseError;

pub struct DefaultAstNode {
    next: Box<dyn AstNode>,
}

pub trait AstNode {
    fn generate(&self) -> Result<u64, ParseError>;
    fn next(&self) -> Option<&dyn AstNode>;
}

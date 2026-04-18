use crate::assembler::parser::errors::parse_errors::ParseError;

pub trait AstNode {
    fn generate(&self) -> Result<u64, ParseError>; 
}

use crate::assembler::parser::errors::parse_errors::ParseError;

pub struct DefaultAstNode {
    pub next: Option<Box<dyn AstNode>>,
}

pub trait AstNode {
    fn generate(&self) -> Result<u64, ParseError>;
    fn next(&self) -> Option<&dyn AstNode>;
    fn set_next(&mut self, node: Box<dyn AstNode>);
}

impl AstNode for DefaultAstNode {
    fn generate(&self) -> Result<u64, ParseError> {
        return Ok(0);
    }

    fn next(&self) -> Option<&dyn AstNode> {
        self.next.as_ref().map(|b| b.as_ref())
    }

    fn set_next(&mut self, node: Box<dyn AstNode>) {
        self.next = Some(node);
    }
}

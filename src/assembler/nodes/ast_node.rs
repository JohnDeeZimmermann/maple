use crate::assembler::{nodes::mov_node::MovNode, parser::errors::parse_errors::ParseError};

pub enum AstNode {
    Mov(MovNode),
}

impl AstNode {
    pub fn generate(&self) -> Result<u64, ParseError> {
        match self {
            AstNode::Mov(node) => node.generate(),
        }
    }
}

use crate::assembler::{
    nodes::{ast_node::AstNode, ast_value::AstValue, instruction::Instruction},
    parser::{errors::parse_errors::ParseError, utils::mask_from_right},
};

pub struct MovInstructionNode {
    next: Box<dyn AstNode>,
    instruction: Instruction,
    target_register: AstValue,
    source_value: AstValue,
}

impl AstNode for MovInstructionNode {
    fn generate(&self) -> Result<u64, ParseError> {
        let details = self.instruction.details();

        let opcode = mask_from_right(details.opcode, 4);
        let option = mask_from_right(details.option, 1);
        let target_register = mask_from_right(self.target_register.to_register_index()? as u64, 4);
        let source_value = mask_from_right(
            self.source_value.to_potential_register_argument(51, true)?,
            51,
        );

        let result = (opcode << 56) | (option << 55) | (target_register << 51) | source_value;

        Ok(result)
    }

    fn next(&self) -> Option<&dyn AstNode> {
        return Some(self.next.as_ref());
    }
}

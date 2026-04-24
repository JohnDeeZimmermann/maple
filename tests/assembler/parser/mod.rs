use maple::assembler::lexer::token::Token;
use maple::assembler::nodes::instruction::Instruction;
use maple::assembler::parser::errors::parse_errors::ParseError;
use maple::assembler::parser::parser::Parser;

#[path = "primitives.rs"]
mod primitives;

fn parser(tokens: Vec<Token>) -> Parser {
    Parser::new(tokens)
}

fn assert_parser_consumed_to(parser: &Parser, token: Token) {
    assert_eq!(parser.peek(), &token);
}

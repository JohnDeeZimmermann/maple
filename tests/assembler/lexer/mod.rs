use maple::assembler::lexer::lexer::lex;
use maple::assembler::lexer::token::Token;

#[path = "numbers.rs"]
mod numbers;
#[path = "strings.rs"]
mod strings;
#[path = "comments.rs"]
mod comments;
#[path = "whitespace.rs"]
mod whitespace;
#[path = "identifiers.rs"]
mod identifiers;
#[path = "classification.rs"]
mod classification;
#[path = "symbols.rs"]
mod symbols;

fn assert_instruction(token: &Token, expected: &str) {
    assert!(matches!(token, Token::Instruction(value) if value == expected));
}

fn assert_pre_assembler_instruction(token: &Token, expected: &str) {
    assert!(matches!(token, Token::PreAssemblerInstruction(value) if value == expected));
}

fn assert_identifier(token: &Token, expected: &str) {
    assert!(matches!(token, Token::Identifier(value) if value == expected));
}

fn assert_number(token: &Token, expected: i64) {
    assert!(matches!(token, Token::Number(value) if *value == expected));
}

fn assert_string(token: &Token, expected: &str) {
    assert!(matches!(token, Token::String(value) if value == expected));
}

fn assert_eol(token: &Token) {
    assert!(matches!(token, Token::EOL));
}

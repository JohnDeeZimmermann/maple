use super::{assert_parser_consumed_to, parser, Instruction, ParseError, Token};

#[test]
fn advance_returns_current_token_and_moves_to_next_token() {
    let mut parser = parser(vec![
        Token::Number(7),
        Token::Identifier("next".to_string()),
    ]);

    assert_eq!(parser.advance(), Token::Number(7));
    assert_parser_consumed_to(&parser, Token::Identifier("next".to_string()));
}

#[test]
fn peek_does_not_consume_token() {
    let parser = parser(vec![Token::String("same".to_string())]);

    assert_eq!(parser.peek(), &Token::String("same".to_string()));
    assert_eq!(parser.peek(), &Token::String("same".to_string()));
}

#[test]
fn expect_consumes_matching_token() {
    let mut parser = parser(vec![Token::Comma, Token::Number(1)]);

    assert!(parser.expect(Token::Comma).is_ok());
    assert_parser_consumed_to(&parser, Token::Number(1));
}

#[test]
fn expect_returns_syntax_error_without_consuming_mismatched_token() {
    let mut parser = parser(vec![Token::Identifier("found".to_string())]);

    match parser.expect(Token::Number(5)) {
        Err(ParseError::SyntaxError { expected, found }) => {
            assert_eq!(expected, "Number(5)");
            assert_eq!(found, "Identifier(\"found\")");
        }
        other => panic!("expected syntax error, got {other:?}"),
    }
    assert_parser_consumed_to(&parser, Token::Identifier("found".to_string()));
}

#[test]
fn parse_number_returns_number_and_consumes_token() {
    let mut parser = parser(vec![Token::Number(-42), Token::EOL]);

    assert_eq!(parser.parse_number().unwrap(), -42);
    assert_parser_consumed_to(&parser, Token::EOL);
}

#[test]
fn parse_number_errors_on_non_number_and_still_consumes_token() {
    let mut parser = parser(vec![
        Token::String("not a number".to_string()),
        Token::Number(1),
    ]);

    match parser.parse_number() {
        Err(ParseError::NumberExpectedError { found }) => {
            assert_eq!(found, "String(\"not a number\")");
        }
        other => panic!("expected number error, got {other:?}"),
    }
    assert_parser_consumed_to(&parser, Token::Number(1));
}

#[test]
fn parse_instruction_returns_instruction_case_insensitively() {
    let mut parser = parser(vec![Token::Instruction("mov".to_string()), Token::EOL]);

    assert_eq!(parser.parse_instruction().unwrap(), Instruction::MOV);
    assert_parser_consumed_to(&parser, Token::EOL);
}

#[test]
fn parse_instruction_errors_on_unknown_instruction_and_consumes_token() {
    let mut parser = parser(vec![Token::Instruction("WAT".to_string()), Token::EOL]);

    match parser.parse_instruction() {
        Err(ParseError::IllegalArgumentError(message)) => {
            assert_eq!(message, "unknown instruction: WAT");
        }
        other => panic!("expected illegal argument error, got {other:?}"),
    }
    assert_parser_consumed_to(&parser, Token::EOL);
}

#[test]
fn parse_instruction_errors_on_wrong_token_type_and_consumes_token() {
    let mut parser = parser(vec![Token::Identifier("MOV".to_string()), Token::EOL]);

    match parser.parse_instruction() {
        Err(ParseError::InstructionExpectedError { found }) => {
            assert_eq!(found, "Identifier(\"MOV\")");
        }
        other => panic!("expected instruction error, got {other:?}"),
    }
    assert_parser_consumed_to(&parser, Token::EOL);
}

#[test]
fn parse_pre_assembler_instruction_returns_name() {
    let mut parser = parser(vec![
        Token::PreAssemblerInstruction("include".to_string()),
        Token::EOL,
    ]);

    assert_eq!(parser.parse_pre_assembler_instruction().unwrap(), "include");
    assert_parser_consumed_to(&parser, Token::EOL);
}

#[test]
fn parse_pre_assembler_instruction_errors_on_wrong_token_type_and_consumes_token() {
    let mut parser = parser(vec![Token::Instruction("include".to_string()), Token::EOL]);

    match parser.parse_pre_assembler_instruction() {
        Err(ParseError::PreAssemblerInstructionExpectedError { found }) => {
            assert_eq!(found, "Instruction(\"include\")");
        }
        other => panic!("expected pre-assembler instruction error, got {other:?}"),
    }
    assert_parser_consumed_to(&parser, Token::EOL);
}

#[test]
fn parse_identifier_returns_name() {
    let mut parser = parser(vec![Token::Identifier("label".to_string()), Token::EOL]);

    assert_eq!(parser.parse_identifier().unwrap(), "label");
    assert_parser_consumed_to(&parser, Token::EOL);
}

#[test]
fn parse_identifier_errors_on_wrong_token_type_and_consumes_token() {
    let mut parser = parser(vec![Token::Number(99), Token::EOL]);

    match parser.parse_identifier() {
        Err(ParseError::IdentifierExpectedError { found }) => {
            assert_eq!(found, "Number(99)");
        }
        other => panic!("expected identifier error, got {other:?}"),
    }
    assert_parser_consumed_to(&parser, Token::EOL);
}

#[test]
fn parse_string_returns_value() {
    let mut parser = parser(vec![Token::String("Hello".to_string()), Token::EOL]);

    assert_eq!(parser.parse_string().unwrap(), "Hello");
    assert_parser_consumed_to(&parser, Token::EOL);
}

#[test]
fn parse_string_errors_on_wrong_token_type_and_consumes_token() {
    let mut parser = parser(vec![Token::Identifier("Hello".to_string()), Token::EOL]);

    match parser.parse_string() {
        Err(ParseError::StringExpectedError { found }) => {
            assert_eq!(found, "Identifier(\"Hello\")");
        }
        other => panic!("expected string error, got {other:?}"),
    }
    assert_parser_consumed_to(&parser, Token::EOL);
}

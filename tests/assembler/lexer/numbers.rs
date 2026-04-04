use super::{assert_eol, assert_identifier, assert_instruction, assert_number, lex, Token};

#[test]
fn lexes_decimal_number_literal() {
    let tokens = lex("ADDI r1, r2, #42\n");

    assert_instruction(&tokens[0], "ADDI");
    assert_identifier(&tokens[1], "r1");
    assert!(matches!(tokens[2], Token::Comma));
    assert_identifier(&tokens[3], "r2");
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(tokens[5], Token::Hash));
    assert_number(&tokens[6], 42);
    assert_eol(&tokens[7]);
}

#[test]
fn lexes_decimal_number_with_underscores() {
    let tokens = lex("MOV r0, #1_024_000\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "r0");
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Hash));
    assert_number(&tokens[4], 1_024_000);
    assert_eol(&tokens[5]);
}

#[test]
fn lexes_hex_number_with_lowercase_prefix() {
    let tokens = lex("MOV r0, #0xff\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "r0");
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Hash));
    assert_number(&tokens[4], 255);
    assert_eol(&tokens[5]);
}

#[test]
fn lexes_hex_number_with_uppercase_prefix_and_underscores() {
    let tokens = lex("MOV r0, #0XAB_CD\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "r0");
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Hash));
    assert_number(&tokens[4], 43_981);
    assert_eol(&tokens[5]);
}

#[test]
fn lexes_binary_number_with_lowercase_prefix() {
    let tokens = lex("MOV r0, #0b101010\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "r0");
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Hash));
    assert_number(&tokens[4], 42);
    assert_eol(&tokens[5]);
}

#[test]
fn lexes_binary_number_with_uppercase_prefix_and_underscores() {
    let tokens = lex("MOV r0, #0B1010_0110\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "r0");
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Hash));
    assert_number(&tokens[4], 166);
    assert_eol(&tokens[5]);
}

#[test]
fn lexes_zero_number_literal() {
    let tokens = lex("MOV r0, #0\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "r0");
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Hash));
    assert_number(&tokens[4], 0);
    assert_eol(&tokens[5]);
}

#[test]
fn lexes_numbers_across_multiple_lines() {
    let tokens = lex("MOV r0, #1\nMOV r1, #0x10\nMOV r2, #0b11\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "r0");
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Hash));
    assert_number(&tokens[4], 1);
    assert_eol(&tokens[5]);

    assert_instruction(&tokens[6], "MOV");
    assert_identifier(&tokens[7], "r1");
    assert!(matches!(tokens[8], Token::Comma));
    assert!(matches!(tokens[9], Token::Hash));
    assert_number(&tokens[10], 16);
    assert_eol(&tokens[11]);

    assert_instruction(&tokens[12], "MOV");
    assert_identifier(&tokens[13], "r2");
    assert!(matches!(tokens[14], Token::Comma));
    assert!(matches!(tokens[15], Token::Hash));
    assert_number(&tokens[16], 3);
    assert_eol(&tokens[17]);
}

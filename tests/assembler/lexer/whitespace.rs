use super::{assert_eol, assert_identifier, assert_instruction, assert_number, lex, Token};

#[test]
fn ignores_spaces_tabs_and_carriage_returns_between_tokens() {
    let tokens = lex("\tMOV\tr0,\r\tr1,\t#1\r\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "r0");
    assert!(matches!(tokens[2], Token::Comma));
    assert_identifier(&tokens[3], "r1");
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(tokens[5], Token::Hash));
    assert_number(&tokens[6], 1);
    assert_eol(&tokens[7]);
}

#[test]
fn emits_eol_for_blank_lines() {
    let tokens = lex("\n\nMOV r0, #1\n");

    assert_eol(&tokens[0]);
    assert_eol(&tokens[1]);
    assert_instruction(&tokens[2], "MOV");
    assert_identifier(&tokens[3], "r0");
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(tokens[5], Token::Hash));
    assert_number(&tokens[6], 1);
    assert_eol(&tokens[7]);
}

#[test]
fn handles_indented_multiline_program() {
    let tokens = lex("  MOV r0, #1\n\tADDI r1, r0, #2\n  SUBI r2, r1, #1\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "r0");
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Hash));
    assert_number(&tokens[4], 1);
    assert_eol(&tokens[5]);

    assert_instruction(&tokens[6], "ADDI");
    assert_identifier(&tokens[7], "r1");
    assert!(matches!(tokens[8], Token::Comma));
    assert_identifier(&tokens[9], "r0");
    assert!(matches!(tokens[10], Token::Comma));
    assert!(matches!(tokens[11], Token::Hash));
    assert_number(&tokens[12], 2);
    assert_eol(&tokens[13]);

    assert_instruction(&tokens[14], "SUBI");
    assert_identifier(&tokens[15], "r2");
    assert!(matches!(tokens[16], Token::Comma));
    assert_identifier(&tokens[17], "r1");
    assert!(matches!(tokens[18], Token::Comma));
    assert!(matches!(tokens[19], Token::Hash));
    assert_number(&tokens[20], 1);
    assert_eol(&tokens[21]);
}

#[test]
fn ignores_trailing_whitespace_before_newline() {
    let tokens = lex("MOV r0, #1   \t\r\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "r0");
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Hash));
    assert_number(&tokens[4], 1);
    assert_eol(&tokens[5]);
}

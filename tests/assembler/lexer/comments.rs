use super::{assert_eol, assert_identifier, assert_instruction, assert_number, lex, Token};

#[test]
fn lexes_inline_comment_after_instruction() {
    let tokens = lex("MOV r0, #1 // load constant\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "r0");
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Hash));
    assert_number(&tokens[4], 1);
    assert_eol(&tokens[5]);
}

#[test]
fn lexes_comment_only_line_as_eol() {
    let tokens = lex("// comment only line\n");

    assert_eol(&tokens[0]);
}

#[test]
fn lexes_multiple_comment_lines_and_code() {
    let tokens = lex("// first\n// second\nMOV r0, #3\n");

    assert_eol(&tokens[0]);
    assert_eol(&tokens[1]);
    assert_instruction(&tokens[2], "MOV");
    assert_identifier(&tokens[3], "r0");
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(tokens[5], Token::Hash));
    assert_number(&tokens[6], 3);
    assert_eol(&tokens[7]);
}

#[test]
fn comments_do_not_consume_following_line() {
    let tokens = lex("MOV r0, #1 // line one\nADDI r1, r1, #2\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "r0");
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Hash));
    assert_number(&tokens[4], 1);
    assert_eol(&tokens[5]);

    assert_instruction(&tokens[6], "ADDI");
    assert_identifier(&tokens[7], "r1");
    assert!(matches!(tokens[8], Token::Comma));
    assert_identifier(&tokens[9], "r1");
    assert!(matches!(tokens[10], Token::Comma));
    assert!(matches!(tokens[11], Token::Hash));
    assert_number(&tokens[12], 2);
    assert_eol(&tokens[13]);
}

#[test]
fn comment_at_eof_has_no_eol_token() {
    let tokens = lex("MOV r0, #1 // trailing comment without newline");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "r0");
    assert!(matches!(tokens[2], Token::Comma));
    assert!(matches!(tokens[3], Token::Hash));
    assert_number(&tokens[4], 1);
    assert_eq!(tokens.len(), 5);
}

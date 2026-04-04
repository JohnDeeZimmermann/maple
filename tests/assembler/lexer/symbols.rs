use super::{
    assert_eol, assert_identifier, assert_instruction, assert_number,
    assert_pre_assembler_instruction, lex, Token,
};

#[test]
fn lexes_plus_and_minus_in_label_offsets() {
    let tokens = lex("B @Start+1\nB @Start-2\n");

    assert_instruction(&tokens[0], "B");
    assert!(matches!(tokens[1], Token::At));
    assert_identifier(&tokens[2], "Start");
    assert!(matches!(tokens[3], Token::Plus));
    assert_number(&tokens[4], 1);
    assert_eol(&tokens[5]);

    assert_instruction(&tokens[6], "B");
    assert!(matches!(tokens[7], Token::At));
    assert_identifier(&tokens[8], "Start");
    assert!(matches!(tokens[9], Token::Minus));
    assert_number(&tokens[10], 2);
    assert_eol(&tokens[11]);
}

#[test]
fn lexes_commas_in_three_parameter_instruction() {
    let tokens = lex("ADDI r1, r2, #3\n");

    assert_instruction(&tokens[0], "ADDI");
    assert_identifier(&tokens[1], "r1");
    assert!(matches!(tokens[2], Token::Comma));
    assert_identifier(&tokens[3], "r2");
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(tokens[5], Token::Hash));
    assert_number(&tokens[6], 3);
    assert_eol(&tokens[7]);
}

#[test]
fn lexes_colon_dollar_in_both_orders() {
    let tokens = lex("$: file MAIN\n:$ export START\n");

    assert!(matches!(tokens[0], Token::ColonDollar));
    assert_pre_assembler_instruction(&tokens[1], "file");
    assert_identifier(&tokens[2], "MAIN");
    assert_eol(&tokens[3]);

    assert!(matches!(tokens[4], Token::ColonDollar));
    assert_pre_assembler_instruction(&tokens[5], "export");
    assert_identifier(&tokens[6], "START");
    assert_eol(&tokens[7]);
}

#[test]
fn lexes_dot_prefixed_label_followed_by_instruction_same_line() {
    let tokens = lex(".START MOV r0, #7\n");

    assert!(matches!(tokens[0], Token::Dot));
    assert_identifier(&tokens[1], "START");
    assert_instruction(&tokens[2], "MOV");
    assert_identifier(&tokens[3], "r0");
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(tokens[5], Token::Hash));
    assert_number(&tokens[6], 7);
    assert_eol(&tokens[7]);
}

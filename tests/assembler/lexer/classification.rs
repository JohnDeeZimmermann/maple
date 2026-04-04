use super::{
    assert_eol, assert_identifier, assert_instruction, assert_pre_assembler_instruction, lex, Token,
};

#[test]
fn classifies_instructions_case_insensitively() {
    let tokens = lex("addi r1, r2, #1\n");

    assert_instruction(&tokens[0], "addi");
}

#[test]
fn classifies_pre_assembler_instructions_case_insensitively() {
    let tokens = lex("$: IMPORT lib/main.masm AS MAIN\n");

    assert!(matches!(tokens[0], Token::ColonDollar));
    assert_pre_assembler_instruction(&tokens[1], "IMPORT");
    assert_identifier(&tokens[2], "lib/main");
    assert!(matches!(tokens[3], Token::Dot));
    assert_identifier(&tokens[4], "masm");
    assert_pre_assembler_instruction(&tokens[5], "AS");
    assert_identifier(&tokens[6], "MAIN");
    assert_eol(&tokens[7]);
}

#[test]
fn keeps_identifier_after_dot_even_if_it_matches_instruction_name() {
    let tokens = lex(".ADDI\n");

    assert!(matches!(tokens[0], Token::Dot));
    assert_identifier(&tokens[1], "ADDI");
    assert_eol(&tokens[2]);
}

#[test]
fn keeps_identifier_after_at_even_if_it_matches_instruction_name() {
    let tokens = lex("B @MOV\n");

    assert_instruction(&tokens[0], "B");
    assert!(matches!(tokens[1], Token::At));
    assert_identifier(&tokens[2], "MOV");
    assert_eol(&tokens[3]);
}

#[test]
fn keeps_non_instruction_identifiers_as_identifiers() {
    let tokens = lex("customSymbol\n");

    assert_identifier(&tokens[0], "customSymbol");
    assert_eol(&tokens[1]);
}

#[test]
fn classifies_tokens_correctly_across_multiple_lines() {
    let tokens = lex(".ENTRY ADDI r1, r2, #1\nB @ENTRY\n$: export ENTRY\n");

    assert!(matches!(tokens[0], Token::Dot));
    assert_identifier(&tokens[1], "ENTRY");
    assert_instruction(&tokens[2], "ADDI");
    assert_identifier(&tokens[3], "r1");
    assert!(matches!(tokens[4], Token::Comma));
    assert_identifier(&tokens[5], "r2");
    assert!(matches!(tokens[6], Token::Comma));
    assert!(matches!(tokens[7], Token::Hash));
    assert!(matches!(tokens[8], Token::Number(1)));
    assert_eol(&tokens[9]);

    assert_instruction(&tokens[10], "B");
    assert!(matches!(tokens[11], Token::At));
    assert_identifier(&tokens[12], "ENTRY");
    assert_eol(&tokens[13]);

    assert!(matches!(tokens[14], Token::ColonDollar));
    assert_pre_assembler_instruction(&tokens[15], "export");
    assert_identifier(&tokens[16], "ENTRY");
    assert_eol(&tokens[17]);
}

use super::{
    assert_eol, assert_identifier, assert_instruction, assert_pre_assembler_instruction, lex, Token,
};

#[test]
fn lexes_register_names_and_leading_underscore_identifiers() {
    let tokens = lex("MOV _result, r1\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "_result");
    assert!(matches!(tokens[2], Token::Comma));
    assert_identifier(&tokens[3], "r1");
    assert_eol(&tokens[4]);
}

#[test]
fn lexes_identifiers_containing_digits() {
    let tokens = lex("MOV value2, r0\n");

    assert_instruction(&tokens[0], "MOV");
    assert_identifier(&tokens[1], "value2");
    assert!(matches!(tokens[2], Token::Comma));
    assert_identifier(&tokens[3], "r0");
    assert_eol(&tokens[4]);
}

#[test]
fn lexes_forward_slash_identifier_in_include_path() {
    let tokens = lex("$: include libs/math/core.masm\n");

    assert!(matches!(tokens[0], Token::ColonDollar));
    assert_pre_assembler_instruction(&tokens[1], "include");
    assert_identifier(&tokens[2], "libs/math/core");
    assert!(matches!(tokens[3], Token::Dot));
    assert_identifier(&tokens[4], "masm");
    assert_eol(&tokens[5]);
}

#[test]
fn lexes_backslash_identifier_in_include_path() {
    let tokens = lex("$: include libs\\core\\math.masm\n");

    assert!(matches!(tokens[0], Token::ColonDollar));
    assert_pre_assembler_instruction(&tokens[1], "include");
    assert_identifier(&tokens[2], "libs\\core\\math");
    assert!(matches!(tokens[3], Token::Dot));
    assert_identifier(&tokens[4], "masm");
    assert_eol(&tokens[5]);
}

#[test]
fn lexes_multiline_import_with_alias_and_label_reference() {
    let tokens = lex("$: import libs/math/core.masm as MATH\nB @MATH.ADD\n");

    assert!(matches!(tokens[0], Token::ColonDollar));
    assert_pre_assembler_instruction(&tokens[1], "import");
    assert_identifier(&tokens[2], "libs/math/core");
    assert!(matches!(tokens[3], Token::Dot));
    assert_identifier(&tokens[4], "masm");
    assert_pre_assembler_instruction(&tokens[5], "as");
    assert_identifier(&tokens[6], "MATH");
    assert_eol(&tokens[7]);

    assert_instruction(&tokens[8], "B");
    assert!(matches!(tokens[9], Token::At));
    assert_identifier(&tokens[10], "MATH");
    assert!(matches!(tokens[11], Token::Dot));
    assert_identifier(&tokens[12], "ADD");
    assert_eol(&tokens[13]);
}

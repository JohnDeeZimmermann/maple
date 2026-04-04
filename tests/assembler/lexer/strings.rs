use super::{
    assert_eol, assert_identifier, assert_instruction, assert_number, assert_string, lex, Token,
};

#[test]
fn lexes_simple_string_data_line() {
    let tokens = lex("\"Hello\"\n");

    assert_string(&tokens[0], "Hello");
    assert_eol(&tokens[1]);
}

#[test]
fn lexes_empty_string_data_line() {
    let tokens = lex("\"\"\n");

    assert_string(&tokens[0], "");
    assert_eol(&tokens[1]);
}

#[test]
fn lexes_string_with_spaces_and_symbols() {
    let tokens = lex("\"Maple VM: hello, world!\"\n");

    assert_string(&tokens[0], "Maple VM: hello, world!");
    assert_eol(&tokens[1]);
}

#[test]
fn treats_comment_like_text_inside_string_as_plain_string_content() {
    let tokens = lex("\"http://example.test\"\n");

    assert_string(&tokens[0], "http://example.test");
    assert_eol(&tokens[1]);
}

#[test]
fn lexes_multiple_string_lines() {
    let tokens = lex("\"Hello\"\n\"Maple\"\n\"VM\"\n");

    assert_string(&tokens[0], "Hello");
    assert_eol(&tokens[1]);
    assert_string(&tokens[2], "Maple");
    assert_eol(&tokens[3]);
    assert_string(&tokens[4], "VM");
    assert_eol(&tokens[5]);
}

#[test]
fn unterminated_string_stops_at_newline_and_lexing_continues() {
    let tokens = lex("\"Hello\nMOV r0, #1\n");

    assert_string(&tokens[0], "Hello");
    assert_eol(&tokens[1]);
    assert_instruction(&tokens[2], "MOV");
    assert_identifier(&tokens[3], "r0");
    assert!(matches!(tokens[4], Token::Comma));
    assert!(matches!(tokens[5], Token::Hash));
    assert_number(&tokens[6], 1);
    assert_eol(&tokens[7]);
}

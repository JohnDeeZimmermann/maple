use crate::assembler::lexer::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\r' => {
                chars.next();
            }
            '\n' => {
                tokens.push(Token::EOL);
                chars.next();
            }
            '/' => {
                if is_comment_start(&chars) {
                    consume_comment(&mut chars);
                } else {
                    let identifier = read_identifier(&mut chars);
                    if !identifier.is_empty() {
                        tokens.push(Token::Identifier(identifier));
                    } else {
                        chars.next();
                    }
                }
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '@' => {
                tokens.push(Token::At);
                chars.next();
            }
            '#' => {
                tokens.push(Token::Hash);
                chars.next();
            }
            '.' => {
                tokens.push(Token::Dot);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '$' | ':' => {
                if try_consume_colon_dollar(&mut chars) {
                    tokens.push(Token::ColonDollar);
                } else {
                    chars.next();
                }
            }
            '"' => {
                tokens.push(Token::String(read_string(&mut chars)));
            }
            '0'..='9' => {
                if let Some(number) = read_number(&mut chars) {
                    tokens.push(Token::Number(number));
                }
            }
            _ => {
                if is_identifier_start(ch) {
                    let identifier = read_identifier(&mut chars);
                    if !identifier.is_empty() {
                        tokens.push(classify_identifier(identifier, &tokens));
                    }
                } else {
                    chars.next();
                }
            }
        }
    }

    tokens
}

fn is_comment_start(chars: &Peekable<Chars<'_>>) -> bool {
    let mut iter = chars.clone();
    matches!(iter.next(), Some('/')) && matches!(iter.next(), Some('/'))
}

fn consume_comment(chars: &mut Peekable<Chars<'_>>) {
    while let Some(&ch) = chars.peek() {
        if ch == '\n' {
            break;
        }

        chars.next();
    }
}

fn try_consume_colon_dollar(chars: &mut Peekable<Chars<'_>>) -> bool {
    let mut iter = chars.clone();
    match (iter.next(), iter.next()) {
        (Some('$'), Some(':')) | (Some(':'), Some('$')) => {
            chars.next();
            chars.next();
            true
        }
        _ => false,
    }
}

fn read_string(chars: &mut Peekable<Chars<'_>>) -> String {
    chars.next();

    let mut value = String::new();
    while let Some(&ch) = chars.peek() {
        if ch == '"' {
            chars.next();
            break;
        }

        if ch == '\n' {
            break;
        }

        value.push(ch);
        chars.next();
    }

    value
}

fn read_number(chars: &mut Peekable<Chars<'_>>) -> Option<i64> {
    let first = chars.next()?;

    if first == '0' {
        if let Some(&next) = chars.peek() {
            if next == 'x' || next == 'X' {
                chars.next();
                return read_prefixed_number(chars, 16, |ch| ch.is_ascii_hexdigit() || ch == '_');
            }

            if next == 'b' || next == 'B' {
                chars.next();
                return read_prefixed_number(chars, 2, |ch| ch == '0' || ch == '1' || ch == '_');
            }
        }
    }

    let mut literal = String::new();
    literal.push(first);
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() || ch == '_' {
            literal.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    literal.retain(|ch| ch != '_');
    literal.parse::<i64>().ok()
}

fn read_prefixed_number<F>(chars: &mut Peekable<Chars<'_>>, radix: u32, is_digit: F) -> Option<i64>
where
    F: Fn(char) -> bool,
{
    let mut digits = String::new();
    while let Some(&ch) = chars.peek() {
        if is_digit(ch) {
            digits.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    if digits.is_empty() {
        return None;
    }

    digits.retain(|ch| ch != '_');
    i64::from_str_radix(&digits, radix).ok()
}

fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch == '/' || ch == '\\'
}

fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '/' || ch == '\\'
}

fn read_identifier(chars: &mut Peekable<Chars<'_>>) -> String {
    let mut identifier = String::new();
    while let Some(&ch) = chars.peek() {
        if is_identifier_char(ch) {
            identifier.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    identifier
}

fn classify_identifier(identifier: String, tokens: &[Token]) -> Token {
    let follows_label_marker = matches!(tokens.last(), Some(Token::Dot) | Some(Token::At));
    if follows_label_marker {
        return Token::Identifier(identifier);
    }

    if is_instruction(&identifier) {
        Token::Instruction(identifier)
    } else if is_pre_assembler_instruction(&identifier)  {
        Token::PreAssemblerInstruction(identifier)
    } else {
        Token::Identifier(identifier)
    }
}

fn is_instruction(value: &str) -> bool {
    let upper = value.to_ascii_uppercase();

    matches!(
        upper.as_str(),
        "NOP"
            | "MOV"
            | "MVN"
            | "ADDI"
            | "SUBI"
            | "MULI"
            | "DIVI"
            | "ADDF"
            | "SUBF"
            | "MULF"
            | "DIVF"
            | "SGE"
            | "SEQ"
            | "SNE"
            | "SNQ"
            | "SGT"
            | "SLT"
            | "SLE"
            | "CMPI"
            | "CMPF"
            | "RGE"
            | "RLE"
            | "REQ"
            | "RNQ"
            | "RGT"
            | "RLT"
            | "BGE"
            | "BLE"
            | "BEG"
            | "BEQ"
            | "BNQ"
            | "BNE"
            | "BGT"
            | "BLT"
            | "B"
            | "BR"
            | "BL"
            | "LSL"
            | "LSR"
            | "AND"
            | "ORR"
            | "XOR"
            | "LDR"
            | "STR"
            | "POP"
            | "PUSH"
            | "EXIT"
            | "IOR"
            | "IOW"
            | "SWI"
            | "RFI"
    )
}

fn is_pre_assembler_instruction(value: &str) -> bool {
    let upper = value.to_ascii_uppercase();
    
    matches!( upper.as_str(), "IMPORT" | "INCLUDE" | "FILE" | "EXPORT" | "AS" )
}

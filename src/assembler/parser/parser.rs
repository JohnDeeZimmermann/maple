use crate::assembler::nodes::instruction::Instruction;
use crate::assembler::{
    lexer::token::Token, nodes::instruction::instruction_from_name,
    parser::errors::parse_errors::ParseError,
};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn advance(&mut self) -> Token {
        let token = self.tokens[self.position].clone();
        self.position += 1;
        return token;
    }

    /// Returns a reference to the token at the current position.
    ///
    /// Should be used to inspect, i.e. to decide on which parsing-branch to take.
    pub fn peek(&self) -> &Token {
        return &self.tokens[self.position];
    }

    /// If the current token matches the expected, it gets consumed and true is returned.
    /// If the current token does not match, nothing happens and false is returned.
    ///
    /// Should be used for optional tokens.
    fn eat(&mut self, expected: &Token) -> bool {
        if self.peek() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    /// If the current token matches the expected, it gets consumed.
    /// If the current token does not match, an error is thrown.
    ///
    /// Should be used for mandatory tokens.
    pub fn expect(&mut self, expected_token: Token) -> Result<(), ParseError> {
        if self.eat(&expected_token) {
            Ok(())
        } else {
            Err(ParseError::SyntaxError {
                expected: format!("{:?}", expected_token),
                found: format!("{:?}", self.peek()),
            })
        }
    }

    /// Consumes the current token and returns the number value if it is a Number token.
    /// Returns an error if the current token is not a Number.
    pub fn parse_number(&mut self) -> Result<i64, ParseError> {
        match self.advance() {
            Token::Number(number) => Ok(number),
            other => Err(ParseError::NumberExpectedError {
                found: format!("{:?}", other),
            }),
        }
    }

    /// Consumes the current token and returns the Instruction enum if it is an Instruction token.
    /// Returns an error if the current token is not an Instruction or if the instruction name is unknown.
    pub fn parse_instruction(&mut self) -> Result<Instruction, ParseError> {
        match self.advance() {
            Token::Instruction(name) => match instruction_from_name(name.clone()) {
                Some(instruction) => Ok(instruction),
                None => Err(ParseError::IllegalArgumentError(format!(
                    "unknown instruction: {}",
                    name
                ))),
            },
            other => Err(ParseError::InstructionExpectedError {
                found: format!("{:?}", other),
            }),
        }
    }

    /// Consumes the current token and returns the pre-assembler instruction name if it is a PreAssemblerInstruction token.
    /// Returns an error if the current token is not a PreAssemblerInstruction.
    pub fn parse_pre_assembler_instruction(&mut self) -> Result<String, ParseError> {
        match self.advance() {
            Token::PreAssemblerInstruction(name) => Ok(name),
            other => Err(ParseError::PreAssemblerInstructionExpectedError {
                found: format!("{:?}", other),
            }),
        }
    }

    /// Consumes the current token and returns the identifier name if it is an Identifier token.
    /// Returns an error if the current token is not an Identifier.
    pub fn parse_identifier(&mut self) -> Result<String, ParseError> {
        match self.advance() {
            Token::Identifier(name) => Ok(name),
            other => Err(ParseError::IdentifierExpectedError {
                found: format!("{:?}", other),
            }),
        }
    }

    /// Consumes the current token and returns the string value if it is a String token.
    /// Returns an error if the current token is not a String.
    pub fn parse_string(&mut self) -> Result<String, ParseError> {
        match self.advance() {
            Token::String(value) => Ok(value),
            other => Err(ParseError::StringExpectedError {
                found: format!("{:?}", other),
            }),
        }
    }
}

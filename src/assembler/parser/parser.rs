use crate::assembler::lexer::token::Token;

fn get_ast(tokens: &Vec<Token>) {
    let mut tokenStream = tokens.into_iter().peekable();

    while let Some(&token) = tokenStream.peek() {}
}

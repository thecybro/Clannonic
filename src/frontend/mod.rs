
mod lexer;
mod parser;
mod token;
mod ast;

pub use lexer::*;
pub use parser::*;
pub use token::*;
pub use ast::*;

/// To store the location of the characters in the source code
/// and later to use it to display debug info
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}


/// What a token in this language has
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind, // what our lexer gives
    pub lexeme: &'a str, // the actual text that we had lexed
    pub span: Span // how long is it? it's starting and ending position
}


#[derive(Debug, Clone)]
pub struct LexedOutput<'a> {
    pub tokens: Box<[Token<'a>]>
}


#[derive(Debug, Clone)]
pub struct Program {
    // This will allow us to return other stuff as part of the program
    // which are not the part of the ast itself
    pub ast: Box<[Stmt]>
}



use std::{
    env,
    fs,
};

// mod parser;
mod lexer;
// mod evaluater;
// mod store;

use lexer::Lexer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Identifier, 
    Integer,
    Float,
    String,
    
    Plus,
    Minus,
    Star,
    Slash,

    Space,
    
    Equals,
    EqualsEquals,

    GreaterThan,
    LessThan,
    Not,
    
    GreaterOrEquals,
    LessOrEquals,

    LeftParen,
    RightParen,

    LeftBrace,
    RightBrace,

    SemCol,
    Comma,
    
    Let,
    For,
    While,
    
    Fn,
    If,
    Else,
    Return,

    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'src> {
    pub kind: TokenKind, // what our lexer gives
    pub lexeme: &'src str, // the actual text that we had lexed
    pub span: Span // how long is it? it's starting and ending position
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = String::new();

    
    if args.len() > 1 {
        path = args[1].clone();
    } else {
        println!("Clannonic")
    }

    let msg = format!("Couldn't read file {path}");

    let source = fs::read_to_string(path.trim()).expect(&msg);

    let tokens = Lexer::new(&source);

    for token in tokens {
        println!("{token:?}")
    }
}

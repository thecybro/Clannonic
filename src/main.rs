
use std::{
    env,
    fs,
};

mod parser;
mod lexer;
// mod evaluater;
// mod store;

use lexer::Lexer;
use parser::Parser;

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

    DSlash, // //
    TSlash, // ///
    MSlash, // Any number of slashes above 3
    
    Space,
    
    Equals,
    EqualsEquals,

    GreaterThan,
    LessThan,
    Not,
    
    GreaterOrEquals,
    LessOrEquals,

    LParen,
    RParen,

    LBrace,
    RBrace,

    SQuote, // '
    DQuote, // ""
    Col, // :
    SemCol, // ;
    Comma, // ,
    Dot, // .

    Keyword(Keyword),
    
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Let,
    For,
    While,
    
    Fn,
    If,
    Else,
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind, // what our lexer gives
    pub lexeme: &'a str, // the actual text that we had lexed
    pub span: Span // how long is it? it's starting and ending position
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Integer(i64),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Let {
        keyword: Keyword,
        identifier: String,
        expression: Expr
    },
    If {
        keyword: Keyword,
        condition: Expr,
        body: Box<Stmt>,
    }
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

    let mut tokens = vec![];
    
    let lexed = Lexer::new(&source);
    for token in lexed {
        // println!("{token:#?}");
        tokens.push(token)
    }
    let parsed = Parser::new(tokens).parse();
    println!("{parsed:#?}")
}

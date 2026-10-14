//! This is the language I am experimenting with, whose name is not yet
//! decided but I am going with **clannonic** for now.
//! 
//! For now, it's just a test crate but later when it can run/evaluate basic code,
//! I am planning to integrate it with my other project **Clanix** to make it the
//! programatic/scripting layer for it allowing users to automate the tasks
//! in **Clanix** instead of having to do all things manually.
//! 
//! You can find the information about **Clanix**; a developer tool at 
//! [Clanix](https://github.com/Clannon-Labs/Clanix)
//!
//! 
//! Visit [Lexer](./lexer/struct.Lexer.html) to see the lexer documentation
//! 
//! Visit [Parser](./parser/struct.Parser.html) to see the parser documentation

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

/// The token kinds the language recognizes
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

    Arrow, // -> Used to indicate function return value
    
    DSlash, // //
    TSlash, // ///
    MSlash, // Any number of slashes above 3
    
    Space,
    
    Equals,
    EqualsEquals,

    GreaterThan,
    LessThan,
    Not,
    
    ExEquals, // !=
    
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

/// The keywords to create statements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Let, // Done
    For,
    While, // Done
    
    Fn,
    If, // Done
    Else,
    Return,
}

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

/// The operators you can use to compare stuff
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionalOp {
    IsEqualTo,
    IsNotEqualThan,
    IsGreaterThan,
    IsLessThan,
    IsGreaterOrEqualTo,
    IsLessOrEqualTo,
}

/// The binary operators for arithmetic operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// The types of expressions supported
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Integer(i64),
    Identifier(String),
    String(String),
    Conditional {
        /// The reason we use Expr instead of Identifier
        /// because we might have to compare between
        /// any 2 expressions as well, like 
        /// ```
        /// (a + b) >= (c + d)
        /// ```
        /// 
        /// here, the individual arithmetic operation expressions like
        /// (a + b) and (c + d) would ultimately be converted into an
        /// identifier or number but if we had made the left and right hand
        /// sides as not Expr, users wouldn't be able to write that
        left: Box<Expr>,
        operator: ConditionalOp,
        right: Box<Expr>
    },
    Binary {
        /// Obviously the reason we used Expr is because
        /// we need to have an AST which has the operations/
        /// expressions in their own respective nodes,
        /// 
        /// If we had made it 'left: TokenKind' instead of 
        /// `left: Box<Expr>`, we would be supporting only one 
        /// arithmetic operation chain.
        /// 
        /// For example:
        /// ```
        /// // In the case of left/right: TokenKind
        /// let x = a + b // valid
        /// let x = a + b - c // invalid
        /// 
        /// // In the case of left/right: Box<Expr>
        /// let x = a + b // valid
        /// let x = a + b - c //valid
        /// 
        /// // And we can make this chain as long as we want,
        /// // like this is also perfectly valid:
        /// 
        /// let x = a + b - c * d / e
        /// 
        /// // Note tha the space between the expressions is not necessary,
        /// // like we can write this as well
        /// 
        /// let x = a+b-c*d/e // perfectly valid
        /// ```
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    }
}

/// The supported statements and their structures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Let {
        identifier: String,
        expression: Expr
    },
    // I think the support for else if or elif branches is also
    // important here and they should have their own body instead
    // of the if statement having a single body
    If {
        condition: Expr,
        body: Box<[Stmt]>,
    },
    While {
        condition: Expr,
        body: Box<[Stmt]>,
    },
    /// It can't take any arguments yet or return a value, but the functionality will soon
    /// be added
    Fn {
        name: Expr,
        body: Box<[Stmt]>
    }
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

/// A simple glue for the lexer and parser (and evaluater/code generator later)
/// which calls and takes return value from one and gives it to other
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

    let tokens = Lexer::new(&source).get_lexed();
    // println!("{tokens:#?}");

    let parsed = Parser::new(tokens).parse();
    println!("{parsed:#?}")
}

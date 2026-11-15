
mod lexer;
mod parser;
mod token;
mod ast;

pub use lexer::*;
pub use parser::*;
pub use token::*;
pub use ast::*;

#[derive(Debug, Clone)]
pub struct LexedOutput<'a> {
    pub tokens: Box<[Token<'a>]>
}


#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    // This will allow us to return other stuff as part of the program
    // which are not the part of the ast itself
    pub ast: Box<[Stmt]>
}

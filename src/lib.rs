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

mod frontend;
mod backends;
mod language;
mod semantic;

use language::*;
use semantic::*;
use frontend::*;
use backends::*;


#[derive(Debug, PartialEq)]
pub enum Error {
    LexError(LexError),
    ParseError(ParseError),
    TypeError(TypeError),
    EvaluateError(EvalError),
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::ParseError(err)
    }
}

impl From<EvalError> for Error {
    fn from(err: EvalError) -> Self {
        Error::EvaluateError(err)
    }
}

impl From<LexError> for Error {
    fn from(err: LexError) -> Self {
        Error::LexError(err)
    }
}

impl From<TypeError> for Error {
    fn from(err: TypeError) -> Self {
        Error::TypeError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// A simple glue for the lexer and parser (and Interpreter/code generator later)
/// which calls and takes return value from one and gives it to other
pub fn run(source: &str) -> Result<()> {
    let tokens = Lexer::new(&source).lex()?;
    // println!("{tokens:#?}");

    let parsed = Parser::new(tokens).parse()?;
    // println!("{parsed:#?}");

    let checked = TypeChecker::new().check_program(&parsed)?;
    
    let result = Interpreter::new().evaluate(&parsed);

    println!("{result:#?}");

    Ok(())
}


#[test]
fn test_end_to_end() {
    let source = "fn main() -> int {
        let x = 1
        return x
    }
    
    let y = main()";
    
    assert_eq!(
        run(&source),
        Ok(())
    )
}
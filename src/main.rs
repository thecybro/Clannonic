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

use std::{
    env,
    fs,
};

mod frontend;
mod runtime;

use frontend::Lexer;
use frontend::{Parser, ParseError};
use runtime::{Evaluater, EvalError};

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
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

/// A simple glue for the lexer and parser (and evaluater/code generator later)
/// which calls and takes return value from one and gives it to other
fn main() -> Result<(), Error> {
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

    let parsed = Parser::new(tokens).parse()?;
    // println!("{parsed:#?}");
    
    // let _ = Evaluater::new(parsed).evaluate()?;
    
    Ok(())
}

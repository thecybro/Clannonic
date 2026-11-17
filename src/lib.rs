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
mod tests;
pub use backends::{
    EvalError,
    Interpreter,
};
pub use frontends::{
    LexError,
    ParseError,
    TypeError,

    Lexer,
    Parser,
    Type,
    TypeChecker,
    TypedProgram
};


#[derive(Debug, PartialEq)]
pub enum Error {
    LexError(LexError),
    ParseError(ParseError),
    TypeError(TypeError),
    EvalError(EvalError),
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::ParseError(err)
    }
}

impl From<EvalError> for Error {
    fn from(err: EvalError) -> Self {
        Error::EvalError(err)
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

    let program = Parser::new(tokens).parse()?;
    // println!("{program:#?}");

    let typed_program = TypeChecker::new().check_program(&program)?;

    // println!("{typed_program:#?}");
    
    // The checker currently validates the RawProgram and also returns
    // the validated frontends::Program. So we are not passing raw parser nodes to a
    // backend; execution will be reconnected when the checker emits Program.

    Interpreter::new().evaluate(&typed_program)?;
    Ok(())
}

#[test]
fn test_multiple_features_at_once() {
    let source = "// This code must be valid before we add more features
    // Including this comment here
    
    let mut resu = add(10, 20);
    
    fn add(x: int, y: int) -> int {
        return x + y;
    }
    
    let mut result: int = add(2, 3);
    result += 4;
    
    if result > 5 {
        result = result - 1;
    }
";

    assert_eq!(run(&source), Ok(()))
}

#[test]
fn test_immutable_assignment_on_unary_expression() {
    let source = "let x = 1;
    x++;";

    assert_eq!(run(&source), Err(Error::TypeError(TypeError::ImmutableAssignment("x".to_string()))))
}
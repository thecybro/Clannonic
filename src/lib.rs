//! The language that aims to give you simplicity like C/zig
//! while removing as much overhead as possible between you and the OS,
//! while also making it harder for you to make mistakes by making
//! you control the data flow explicitly and with intent, so you
//! dont have to fight the borrow checker which gets annoying sometimes

mod tests;
use rupple_frontend::RuppleFrontendError;
pub use rupple_interpreter::{
    EvalError,
    Interpreter,
};
pub use rupple_frontend::{
    LexError,
    ParseError,
    TypeError,

    Lexer,
    Parser,
    Type,
    TypeChecker,
    TypedProgram,
    checked_program,
};


#[derive(Debug, PartialEq)]
pub enum Error {
    Frontend(RuppleFrontendError),
    Interpreter(EvalError),
}

impl From<RuppleFrontendError> for Error {
    fn from(err: RuppleFrontendError) -> Self {
        Error::Frontend(err)
    }
}

impl From<EvalError> for Error {
    fn from(err: EvalError) -> Self {
        Error::Interpreter(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// A simple glue for the lexer and parser (and Interpreter/code generator later)
/// which calls and takes return value from one and gives it to other
pub fn run(source: &str) -> Result<()> {
    let typed_program = checked_program(source)?;
    
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

    assert_eq!(run(&source), Err(
        Error::Frontend(
            RuppleFrontendError::Type(
                    TypeError::ImmutableAssignment("x".to_string())
                )
            )
        )
    )
}
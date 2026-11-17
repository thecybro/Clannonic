#![cfg(test)]

use crate::{
    Result,
    Error,
    
    Lexer,
    Parser,
    Type,
    TypeChecker,
    TypedProgram,
    // Interpreter,
};
use frontends::ArithmeticOp;

pub fn checked_program(source: &str) -> Result<TypedProgram> {
    let tokens = Lexer::new(source)
        .lex()?;
    
    let raw_program = Parser::new(tokens)
        .parse()?;
    
    Ok(TypeChecker::new()
        .check_program(&raw_program)?)
}


#[test]
fn checker_rejects_non_integer_addition() {
    let result = checked_program("let x = false + 2;");

    assert!(matches!(
        result,
        Err(Error::TypeError(
            frontends::TypeError::InvalidArithmeticOperation{
                left: Type::Bool,
                operator: ArithmeticOp::Add,
                right: Type::Int
            }
        ))
    ))
}

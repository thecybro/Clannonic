#![cfg(test)]

use rupple_frontend::{ArithmeticOp, checked_program, RuppleFrontendError, TypeError, Type};


#[test]
fn checker_rejects_non_integer_addition() {
    let result = checked_program("let x = false + 2;");

    assert!(matches!(
        result,
        Err(RuppleFrontendError::Type(
            TypeError::InvalidArithmeticOperation{
                left: Type::Bool,
                operator: ArithmeticOp::Add,
                right: Type::Int
            }
        ))
    ))
}

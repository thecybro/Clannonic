use crate::frontend::{BinaryOp, BooleanType, ConditionalOp};


pub(super) type Result<T> = std::result::Result<T, EvalError>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(BooleanType),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    UndefinedVariable(String),
    DuplicateVariable(String),
    InvalidReturnType(Value),
    DuplicateFunctionDeclaration(String),
    InvalidBinaryOperation {
        left: Value,
        operator: BinaryOp,
        right: Value
    },
    InvalidConditionalOperation {
        left: Value,
        operator: ConditionalOp,
        right: Value
    }
}


use crate::frontend::{BinaryOp, ConditionalOp};


pub(super) type Result<T> = std::result::Result<T, EvalError>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    UndefinedVariable(String),
    DuplicateVariable(String),
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


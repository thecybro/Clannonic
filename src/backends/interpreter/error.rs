
use crate::language::{
    BinaryOp,
    ComparisonOp,
    UnaryOp,
    UnaryFixity,
    AssignOp,
};
use crate::frontend::{
    Stmt,
    Expr,
};
use super::Value;

pub(super) type Result<T> = std::result::Result<T, EvalError>;

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    UndefinedVariable(String),
    DuplicateVariable(String),
    FailedToStoreValue(String),
    InvalidReturnType(Value),
    DuplicateFunctionDeclaration(String),
    UnsupportedStatement(Stmt),
    ExpressionNotCallable {
        error: String,
        value: Value,
    },
    InvalidBinaryOperation {
        left: Value,
        operator: BinaryOp,
        right: Value
    },
    InvalidComparisonOperation {
        left: Value,
        operator: ComparisonOp,
        right: Value
    },
    InvalidUnaryOperation {
        operator: UnaryOp,
        operand: Box<Expr>,
        fixity: UnaryFixity
    },
    InvalidAssignOperation {
        identifier: String,
        operator: AssignOp,
        stored_val: Value,
        expression: Expr
    }
}
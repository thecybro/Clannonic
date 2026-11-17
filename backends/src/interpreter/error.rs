
use frontends::{
    ArithmeticOp,
    ComparisonOp,
    UnaryOp,
    UnaryFixity,
    AssignOp,
};
use frontends::{
    TypedStmt,
    TypedExpr,
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
    UnsupportedStatement(TypedStmt),
    ExpressionNotCallable {
        error: String,
        value: Value,
    },
    InvalidArithmeticOperation {
        left: Value,
        operator: ArithmeticOp,
        right: Value
    },
    InvalidComparisonOperation {
        left: Value,
        operator: ComparisonOp,
        right: Value
    },
    InvalidUnaryOperation {
        operator: UnaryOp,
        operand: Box<TypedExpr>,
        fixity: UnaryFixity
    },
    InvalidAssignOperation {
        identifier: String,
        operator: AssignOp,
        stored_val: Value,
        expression: TypedExpr
    }
}
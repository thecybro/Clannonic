use crate::language::{ArithmeticOp, AssignOp, ComparisonOp, Type, UnaryFixity, UnaryOp};

use super::BooleanVal;

/// Syntax tree produced directly by the parser.
///
/// Raw nodes may still refer to undeclared names, contain invalid type
/// combinations, and call unknown functions. They must be checked by the
/// semantic phase before being passed to a backend.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum RawExpr {
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),
    Boolean(BooleanVal),
    Comparison {
        left: Box<RawExpr>,
        operator: ComparisonOp,
        right: Box<RawExpr>,
    },
    Arithmetic {
        left: Box<RawExpr>,
        operator: ArithmeticOp,
        right: Box<RawExpr>,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<RawExpr>,
        fixity: UnaryFixity,
    },
    FunctionCall {
        name: String,
        arguments: Box<[RawExpr]>,
    },
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum RawStmt {
    Let {
        identifier: String,
        mutable: bool,
        annotation: Option<Type>,
        expression: RawExpr,
    },
    Assign {
        identifier: String,
        operator: AssignOp,
        expression: RawExpr,
    },
    If {
        condition: RawExpr,
        body: Box<[RawStmt]>,
    },
    While {
        condition: RawExpr,
        body: Box<[RawStmt]>,
    },
    Function {
        name: String,
        parameters: Box<[RawFunctionParameter]>,
        return_type: Type,
        body: Box<[RawStmt]>,
    },
    Return(Option<RawExpr>),
    Expression(RawExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawFunctionParameter {
    pub name: String,
    pub data_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawProgram {
    pub ast: Box<[RawStmt]>,
}

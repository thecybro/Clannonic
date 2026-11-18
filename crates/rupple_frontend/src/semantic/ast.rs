use crate::language::{ArithmeticOp, AssignOp, ComparisonOp, Type, UnaryFixity, UnaryOp};

use super::super::parse::BooleanVal;

/// Validated program representation consumed by language backends.
///
/// Unlike the parser's Raw* nodes, these nodes are produced only after
/// semantic analysis has accepted the program.
#[derive(Debug, Clone, PartialEq)]
pub struct TypedProgram {
    pub ast: Box<[TypedStmt]>,
}

/// Alias for callers that prefer the name Ast for the validated tree.
// pub type Ast = TypedProgram;

#[derive(Debug, Clone, PartialEq)]
pub struct TypedExpr {
    pub kind: TypedExprKind,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum TypedExprKind {
    Integer(i64),
    Identifier(String),
    Boolean(BooleanVal),
    Comparison {
        left: Box<TypedExpr>,
        operator: ComparisonOp,
        right: Box<TypedExpr>,
    },
    Arithmetic {
        left: Box<TypedExpr>,
        operator: ArithmeticOp,
        right: Box<TypedExpr>,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<TypedExpr>,
        fixity: UnaryFixity,
    },
    FunctionCall {
        name: String,
        arguments: Box<[TypedExpr]>,
    },
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum TypedStmt {
    Let {
        identifier: String,
        mutable: bool,
        ty: Type,
        expression: TypedExpr,
    },
    Assign {
        identifier: String,
        operator: AssignOp,
        expression: TypedExpr,
    },
    If {
        condition: TypedExpr,
        body: Box<[TypedStmt]>,
    },
    While {
        condition: TypedExpr,
        body: Box<[TypedStmt]>,
    },
    Function {
        name: String,
        parameters: Box<[FunctionParameter]>,
        return_type: Type,
        body: Box<[TypedStmt]>,
    },
    Return(Option<TypedExpr>),
    Expression(TypedExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    pub name: String,
    pub ty: Type,
}

impl std::fmt::Display for FunctionParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.name, self.ty)
    }
}

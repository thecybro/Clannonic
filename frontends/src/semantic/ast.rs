use crate::language::{ArithmeticOp, AssignOp, ComparisonOp, Type, UnaryFixity, UnaryOp};

use super::super::parser::BooleanVal;

/// Validated program representation consumed by language backends.
///
/// Unlike the parser's Raw* nodes, these nodes are produced only after
/// semantic analysis has accepted the program.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub ast: Box<[Stmt]>,
}

/// Alias for callers that prefer the name Ast for the validated tree.
pub type Ast = Program;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Expr {
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),
    Boolean(BooleanVal),
    Comparison {
        left: Box<Expr>,
        operator: ComparisonOp,
        right: Box<Expr>,
    },
    Arithmetic {
        left: Box<Expr>,
        operator: ArithmeticOp,
        right: Box<Expr>,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
        fixity: UnaryFixity,
    },
    FunctionCall {
        name: String,
        arguments: Box<[Expr]>,
    },
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Stmt {
    Let {
        identifier: String,
        mutable: bool,
        ty: Type,
        expression: Expr,
    },
    Assign {
        identifier: String,
        operator: AssignOp,
        expression: Expr,
    },
    If {
        condition: Expr,
        body: Box<[Stmt]>,
    },
    While {
        condition: Expr,
        body: Box<[Stmt]>,
    },
    Function {
        name: String,
        parameters: Box<[FunctionParameter]>,
        return_type: Type,
        body: Box<[Stmt]>,
    },
    Return(Option<Expr>),
    Expression(Expr),
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

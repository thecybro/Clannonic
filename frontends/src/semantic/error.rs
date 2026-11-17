
use crate::{RawExpr, UnaryFixity};

use crate::{ArithmeticOp, Type, language::{ComparisonOp, UnaryOp}};

pub(super) type Result<T> = std::result::Result<T, TypeError>;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    UndefinedVariable(String),

    UndefinedFunction(String),
    
    DuplicateVariable(String),

    DuplicateFunctionDeclaration(String),

    InvalidMutationTarget,
    
    ImmutableAssignment(String),

    UnsupportedExpression(RawExpr),
    
    TypeMismatch {
        expected: Type,
        found: Type
    },

    InvalidArithmeticOperation {
        left: Type,
        operator: ArithmeticOp,
        right: Type,
    },

    InvalidComparisonOperation {
        left: Type,
        operator: ComparisonOp,
        right: Type,
    },
    
    InvalidUnaryOperation {
        operator: UnaryOp,
        found: Type,
        fixity: UnaryFixity
    },

    ConditionMustBeBoolean {
        found: Type
    },

    WrongArgumentCount {
        expected: usize,
        found: usize
    },

    InvalidArgumentType {
        parameter: usize,
        expected: Type,
        found: Type,
    },

    ReturnOutsideFunction,

    InvalidReturnType {
        expected: Type,
        found: Type,
    }
}

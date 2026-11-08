
use std::{
    collections::HashMap, rc::Rc,
};

use crate::frontend::{DataType, BooleanType, Expr, FunctionParamater, Stmt};
use crate::frontend::{BinaryOp, ConditionalOp};

pub(super) type Result<T> = std::result::Result<T, EvalError>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(BooleanType),

    Function(Rc<Function>),
    
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{}", num),
            Value::String(string) => write!(f, "{}", string),
            Value::Boolean(bool_type) => {
                if *bool_type == BooleanType::True {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            },
            Value::Null => write!(f, "Null"),
            Value::Function(function) => {
                writeln!(f, "Function:-")?;
                writeln!(f, "Name: {}", function.name)?;

                let params = &function.parameters;
                if !params.is_empty() {
                    writeln!(f, "Parameter(s): ")?;
                    for param in params {
                        writeln!(f, " - {}", param)?;
                    }
                }
                Ok(())
            }
        }
    }
}

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
    InvalidConditionalOperation {
        left: Value,
        operator: ConditionalOp,
        right: Value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnvId(pub usize);

#[derive(Debug, Clone)]
pub struct Environment {
    pub(super) parent: Option<EnvId>,
    pub(super) values: HashMap<String, Value>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub(super) name: String,
    pub(super) parameters: Box<[FunctionParamater]>,
    pub(super) return_type: Option<DataType>,
    pub(super) return_expr: Option<Expr>,
    pub(super) body: Box<[Stmt]>,

    pub(super) closure: EnvId,
}

#[derive(Debug)]
pub struct Interpreter {
    pub(super) environments: Vec<Environment>,
    /// Where the top level declarations would be saved.
    /// It includes user defined identifiers, functions
    /// as well as the builtin functions and std modules
    pub(super) globals: EnvId, 
    pub(super) current: EnvId,
}


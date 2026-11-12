
use std::rc::Rc;

use crate::language::{
    Type,
};
use crate::frontend::{
    BooleanVal,
    FunctionParameter,
    Expr,
    Stmt,
};
use super::EnvId;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Float(f64),
    Integer(i64),
    String(String),
    Boolean(BooleanVal),

    Function(Rc<Function>),

    Unit,
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Flow {
    Continue,
    Return(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub(super) name: String,
    pub(super) parameters: Box<[FunctionParameter]>,
    pub(super) return_type: Type,
    pub(super) body: Box<[Stmt]>,

    pub(super) closure: EnvId,
}


impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(num) => write!(f, "Integer({})", num),
            Value::Float(num) => write!(f, "Float({})", num),
            Value::String(string) => write!(f, "String({})", string),
            Value::Boolean(bool_type) => {
                if *bool_type == BooleanVal::True {
                    write!(f, "Bool(true)")
                } else {
                    write!(f, "Bool(false)")
                }
            },
            Value::Function(function) => {
                writeln!(f, "Function:-")?;
                writeln!(f, "Name: {}", function.name)?;

                let params = &function.parameters;
                if !params.is_empty() {
                    writeln!(f, "Parameter(s): ")?;
                    for param in params {
                        write!(f, " - ")?;
                        write!(f, " {}", param)?;
                    }
                }

                let _type = &function.return_type;
                
                if *_type != Type::Unit {
                    writeln!(f, "Return Type: {:#?}", _type)?;
                }

                // if let Some(ret_expr) = &function.return_expr {
                //     writeln!(f, "{}", ret_expr)?;
                // }
                Ok(())
            },
            Value::Null => write!(f, "Null"),
            Value::Unit => write!(f, "()")
        }
    }
}


use std::rc::Rc;

use rupple_frontend::Type;
use rupple_frontend::{
    BooleanVal,
    FunctionParameter,
    // TypedExpr,
    TypedStmt,
};

use super::EnvId;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
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
    pub(super) body: Box<[TypedStmt]>,

    pub(super) closure: EnvId,
}


impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(num) => write!(f, "Integer({})", num),
            Value::Boolean(bool_type) => {
                if *bool_type == BooleanVal::True {
                    write!(f, "Bool(true)")
                } else {
                    write!(f, "Bool(false)")
                }
            },
            Value::Function(function) => {
                let parameters = &function.parameters;
                let mut params: Vec<String> = Vec::with_capacity(parameters.len());

                if !params.is_empty() {
                    for param in parameters {
                        params.push(param.name.to_string())
                    }
                }
                
                let _type = &function.return_type;
                
                if *_type == Type::Unit {
                    write!(f, "{}({})", function.name, params.join(","))?;
                } else {
                    write!(f, "{}({}) -> {}", function.name, params.join(","), _type)?;
                }
                Ok(())
            },
            Value::Null => write!(f, "Null"),
            Value::Unit => write!(f, "()")
        }
    }
}


use crate::frontend::{
    Stmt,
    Expr,
};

use super::{
    Value,
    Environment,

    Result,
    // EvalError,
};

#[derive(Debug)]
pub struct Evaluater {
    environment: Environment
}

impl Evaluater {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }
    
//     pub fn evaluate(
//         &mut self,
//         statements: &[Stmt],
//     ) -> Result<()> {
// 
//         
//     }

    pub fn eval_expression(
        &mut self,
        expression: &Expr
    ) -> Result<Value> {
        
        match expression {
            Expr::Integer(value) => Ok(Value::Number(*value)),
            Expr::String(value) => Ok(Value::String(value.clone())),
            Expr::Identifier(name) => {
                self.environment.get(name).cloned()
            },
            Expr::Conditional {..} => {
                todo!("Conditional expressions are next!")
            },
            Expr::Binary {..} => {
                todo!("Binary expressions are next!")
            }
        }
    }
}



use crate::frontend::{
    BinaryOp, Expr, Stmt,
};

use super::{
    Value,
    Environment,

    Result,
    EvalError,
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
        &self,
        expression: &Expr
    ) -> Result<Value> {
        
        match expression {
            Expr::Integer(value) => Ok(Value::Number(*value)),
            Expr::String(value) => Ok(Value::String(value.clone())),
            Expr::Identifier(name) => {
                self.environment.get(name).cloned()
            },
            Expr::Binary { left, operator, right } => {
                self.eval_binary_expression(left, operator, right)
            }
            Expr::Conditional {..} => {
                todo!("Conditional expressions are next!")
            },
        }
    }

    fn eval_binary_expression(
        &self,
        left: &Expr,
        operator: &BinaryOp,
        right: &Expr 
    ) -> Result<Value> {

        let left = self.eval_expression(left)?;
        let right = self.eval_expression(right)?;

        match (left, right) {
            (Value::Number(left), Value::Number(right)) => {
                match operator {
                    BinaryOp::Add => Ok(Value::Number(left + right)),
                    BinaryOp::Subtract => Ok(Value::Number(left - right)),
                    BinaryOp::Multiply => Ok(Value::Number(left * right)),
                    BinaryOp::Divide => {
                        if right == 0.0 {
                            return Err(
                                EvalError::InvalidBinaryOperation {
                                    left: Value::Number(left),
                                    operator: BinaryOp::Divide,
                                    right: Value::Number(right)
                                }
                            )
                        }

                        Ok(Value::Number(left / right))
                    },
                }
            },
            (left, right) => {
                Err(
                    EvalError::InvalidBinaryOperation {
                        left,
                        operator: *operator,
                        right
                    }
                )
            }
        }
    }
}

#[test]
fn ok_evaluate_binary_expression_with_identifier() {
    // Create a manual environment so we can manually
    // define/insert values into the variable for testing
    let mut environment = Environment::new();

    // Define the variable "x" with 5.0
    let _ = environment.define("x".to_string(), Value::Number(5.0));
    
    let evaluater = Evaluater {
        environment
    };

    let expression = Expr::Binary {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Integer(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression),
        Ok(Value::Number(15.0))
    );
}

#[test]
fn err_evaluate_binary_expression_with_string_stored_in_identifier() {
    // Create a manual environment so we can manually
    // define/insert values into the variable for testing
    let mut environment = Environment::new();

    // Define the variable "x" with "a_string"
    let _ = environment.define("x".to_string(), Value::String("a_string".to_string()));
    
    let evaluater = Evaluater {
        environment
    };

    let expression = Expr::Binary {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Integer(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression),
        Err(
            EvalError::InvalidBinaryOperation {
                left: Value::String("a_string".to_string()),
                operator: BinaryOp::Add,
                right: Value::Number(10.0)
            }
        )
    );

    let expression2 = Expr::Binary {
        left: Box::new(Expr::Identifier("y".to_string())),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Integer(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression2),
        Err(
            EvalError::UndefinedVariable("y".to_string())
        )
    );
}

#[test]
fn ok_evaluate_normal_binary_expression() {
    let evaluater = Evaluater::new();

    // 10.0 + 20.0 = 30.0
    let expression1 = Expr::Binary {
        left: Box::new(Expr::Integer(10.0)),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Integer(20.0))
    };
    
    assert_eq!(
        evaluater.eval_expression(&expression1),
        Ok(Value::Number(30.0))
    );

    // 20.0 - 10.0 = 10.0
    let expression2 = Expr::Binary {
        left: Box::new(Expr::Integer(20.0)),
        operator: BinaryOp::Subtract,
        right: Box::new(Expr::Integer(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression2),
        Ok(Value::Number(10.0))
    );

    // 20.0 * 10.0 = 200.0
    let expression3 = Expr::Binary {
        left: Box::new(Expr::Integer(20.0)),
        operator: BinaryOp::Multiply,
        right: Box::new(Expr::Integer(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression3),
        Ok(Value::Number(200.0))
    );

    // 20.0 / 10.0 = 2.0
    let expression4 = Expr::Binary {
        left: Box::new(Expr::Integer(20.0)),
        operator: BinaryOp::Divide,
        right: Box::new(Expr::Integer(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression4),
        Ok(Value::Number(2.0))
    );

}

#[test]
fn err_evaluate_divide_by_zero() {
    let evaluater = Evaluater::new();

    let expression1 = Expr::Binary{
        left: Box::new(Expr::Integer(10.0)),
        operator: BinaryOp::Divide,
        right: Box::new(Expr::Integer(0.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression1),
        Err(EvalError::InvalidBinaryOperation {
            left: Value::Number(10.0),
            operator: BinaryOp::Divide,
            right: Value::Number(0.0)
        })
    );
}

#[test]
fn err_evaluate_binary_expression_with_string() {
    let evaluater = Evaluater::new();

    // "something" + 20.0 = Err()
    let expression1 = Expr::Binary {
        left: Box::new(Expr::String("something".to_string())),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Integer(20.0))
    };
    
    assert_eq!(
        evaluater.eval_expression(&expression1),
        Err(EvalError::InvalidBinaryOperation {
            left: Value::String("something".to_string()),
            operator: BinaryOp::Add,
            right: Value::Number(20.0)
        })
    );

    // "something" - 20.0 = Err()
    let expression2 = Expr::Binary {
        left: Box::new(Expr::String("something".to_string())),
        operator: BinaryOp::Subtract,
        right: Box::new(Expr::Integer(20.0))
    };
    
    assert_eq!(
        evaluater.eval_expression(&expression2),
        Err(EvalError::InvalidBinaryOperation {
            left: Value::String("something".to_string()),
            operator: BinaryOp::Subtract,
            right: Value::Number(20.0)
        })
    );

    // "something" * 20.0 = Err()
    let expression3 = Expr::Binary {
        left: Box::new(Expr::String("something".to_string())),
        operator: BinaryOp::Multiply,
        right: Box::new(Expr::Integer(20.0))
    };
    
    assert_eq!(
        evaluater.eval_expression(&expression3),
        Err(EvalError::InvalidBinaryOperation {
            left: Value::String("something".to_string()),
            operator: BinaryOp::Multiply,
            right: Value::Number(20.0)
        })
    );

    // "something" / 20.0 = Err()
    let expression4 = Expr::Binary {
        left: Box::new(Expr::String("something".to_string())),
        operator: BinaryOp::Divide,
        right: Box::new(Expr::Integer(20.0))
    };
    
    assert_eq!(
        evaluater.eval_expression(&expression4),
        Err(EvalError::InvalidBinaryOperation {
            left: Value::String("something".to_string()),
            operator: BinaryOp::Divide,
            right: Value::Number(20.0)
        })
    );

    
}

#[test]
fn ok_evaluate_normal_nested_binary_expression() {
    let evaluater = Evaluater::new();

    let expression = Expr::Binary {
        left: Box::new(Expr::Integer(10.0)),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Binary {
            left: Box::new(Expr::Integer(20.0)),
            operator: BinaryOp::Multiply,
            right: Box::new(Expr::Integer(2.0))
        })
    };

    assert_eq!(
        evaluater.eval_expression(&expression),
        Ok(Value::Number(50.0))
    );
}

#[test]
fn err_evaluate_invalid_binary_expression() {
    let evaluater = Evaluater::new();
    
    let expression = Expr::Binary {
        left: Box::new(Expr::Integer(10.0)),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Binary {
            left: Box::new(Expr::Integer(20.0)),
            operator: BinaryOp::Multiply,
            right: Box::new(Expr::String("invalid".to_string()))
        })
    };
    
    assert_eq!(
        evaluater.eval_expression(&expression),
        Err(EvalError::InvalidBinaryOperation {
            left: Value::Number(20.0),
            operator: BinaryOp::Multiply,
            right: Value::String("invalid".to_string())
        })
    )
}


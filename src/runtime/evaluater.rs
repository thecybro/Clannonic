
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
                let left = self.eval_expression(left)?;
                let right = self.eval_expression(right)?;

                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        let result = match operator {
                            BinaryOp::Add => left + right,
                            BinaryOp::Subtract => left - right,
                            BinaryOp::Multiply => left * right,
                            BinaryOp::Divide => {
                                if right == 0.0 {
                                    return Err(
                                        EvalError::InvalidBinaryOperation {
                                            left: Value::Number(left),
                                            operator: *operator,
                                            right: Value::Number(right)
                                        }
                                    )
                                }

                                left / right
                            },
                        };

                        Ok(Value::Number(result))
                    }
                    (left, right) => Err(EvalError::InvalidBinaryOperation {
                        left,
                        operator: *operator,
                        right,
                    }),
                }
            }
            Expr::Conditional {..} => {
                todo!("Conditional expressions are next!")
            },
        }
    }
}

#[test]
fn ok_evaluate_normal_binary_addition() {
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
fn ok_evaluate_normal_nested_binary_addition() {
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


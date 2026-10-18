
use crate::frontend::{
    BinaryOp, ConditionalOp, Expr, Program, Stmt,
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
    
    pub fn evaluate(
        &mut self,
        program: &Program,
    ) -> Result<()> {

        let statements: Vec<Stmt> = program.ast.to_vec();
        
        for stmt in statements {
            let _ = self.eval_statement(&stmt)?;
        }

        Ok(())
    }

    fn eval_statement(
        &mut self,
        stmt: &Stmt
    ) -> Result<()> {
        
        match stmt {
            Stmt::Let { identifier, expression } => {
                let _ = self.eval_let_statement(identifier, expression)?;
                Ok(())
            },
            Stmt::If { condition, body } => {
                self.eval_if_statement(condition, body)
            },
            Stmt::While { condition, body } => {
                todo!("While statement evaluater will be implemented soon")
            },
            Stmt::Fn { name, return_type, body } => {
                todo!("Fn statement evaluater will be implemented in the upcoming commits")
            }
        }
    }

    fn eval_let_statement(
        &mut self,
        identifier: &str,
        expression: &Expr
    ) -> Result<Value> {

        let value = match expression {
            Expr::Integer(num) => Value::Number(*num),
            Expr::Identifier(ident) => {
                self.environment.get(ident)?.clone()
            },
            Expr::String(a_string) => Value::String(a_string.clone()),
            _ => self.eval_expression(expression)?
        };

        self.environment.define(identifier.to_string(), value)
    }

    fn eval_if_statement(
        &mut self,
        condition: &Expr,
        body: &Box<[Stmt]>
    ) -> Result<()> {

        let condition_result = self.eval_expression(condition)?;

        if condition_result == Value::Boolean(true) {
            for stmt in body.to_vec() {
                self.eval_statement(&stmt)?
            }
        }

        Ok(())

        
    }
    
    fn eval_expression(
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
            Expr::Conditional { left, operator, right } => {
                self.eval_conditional_expression(left, operator, right)
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

    fn eval_conditional_expression(
        &self,
        left: &Expr,
        operator: &ConditionalOp,
        right: &Expr
    ) -> Result<Value> {

        let left = self.eval_expression(left)?;
        let right = self.eval_expression(right)?;

        match (left, right) {
            (Value::Number(left), Value::Number(right)) => {
                match operator {
                    ConditionalOp::IsEqualTo => {
                        Ok(Value::Boolean(left == right))
                    },
                    ConditionalOp::IsNotEqualTo => {
                        Ok(Value::Boolean(left != right))
                    },
                    ConditionalOp::IsGreaterThan => {
                        Ok(Value::Boolean(left > right))
                    },
                    ConditionalOp::IsLessThan => {
                        Ok(Value::Boolean(left < right))
                    },
                    ConditionalOp::IsGreaterOrEqualTo => {
                        Ok(Value::Boolean(left >= right ))
                    },
                    ConditionalOp::IsLessOrEqualTo => {
                        Ok(Value::Boolean(left <= right))
                    }
                }
            },
            (left, right) => {
                return Err(
                    EvalError::InvalidConditionalOperation {
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

#[test]
fn ok_evaluate_conditional_expression() {
    let evaluater = Evaluater::new();

    // 10.0 == 10.0: true
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::Integer(10.0)),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(Expr::Integer(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression1),
        Ok(Value::Boolean(true))
    );

    // 10.0 > 10.0: false
    let expression2 = Expr::Conditional {
        left: Box::new(Expr::Integer(10.0)),
        operator: ConditionalOp::IsGreaterThan,
        right: Box::new(Expr::Integer(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression2),
        Ok(Value::Boolean(false))
    );

    // 10.0 < 10.0: false
    let expression3 = Expr::Conditional {
        left: Box::new(Expr::Integer(10.0)),
        operator: ConditionalOp::IsLessThan,
        right: Box::new(Expr::Integer(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression3),
        Ok(Value::Boolean(false))
    );

    // 10.0 != 10.0: false
    let expression4 = Expr::Conditional {
        left: Box::new(Expr::Integer(10.0)),
        operator: ConditionalOp::IsNotEqualTo,
        right: Box::new(Expr::Integer(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression4),
        Ok(Value::Boolean(false))
    );

    // 10.0 >= 10.0: true
    let expression5 = Expr::Conditional {
        left: Box::new(Expr::Integer(10.0)),
        operator: ConditionalOp::IsGreaterOrEqualTo,
        right: Box::new(Expr::Integer(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression5),
        Ok(Value::Boolean(true))
    );

    // 10.0 <= 10.0: true
    let expression6 = Expr::Conditional {
        left: Box::new(Expr::Integer(10.0)),
        operator: ConditionalOp::IsLessOrEqualTo,
        right: Box::new(Expr::Integer(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression6),
        Ok(Value::Boolean(true))
    );
}

#[test]
fn ok_evaluate_nested_binary_conditional_expression() {
    let evaluater = Evaluater::new();

    // 10.0 == (5.0 + 5.0): true
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::Integer(10.0)),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Integer(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Integer(5.0))
            }
        )
    };

    assert_eq!(
        evaluater.eval_expression(&expression1),
        Ok(Value::Boolean(true))
    );

    // 10.0 < (5.0 + 5.0): true
    let expression2 = Expr::Conditional {
        left: Box::new(Expr::Integer(10.0)),
        operator: ConditionalOp::IsLessThan,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Integer(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Integer(5.0))
            }
        )
    };

    assert_eq!(
        evaluater.eval_expression(&expression2),
        Ok(Value::Boolean(false))
    );

    // 10.0 > (5.0 + (15.0 - 5.0)): false
    let expression3 = Expr::Conditional {
        left: Box::new(Expr::Integer(10.0)),
        operator: ConditionalOp::IsGreaterThan,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Integer(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Integer(15.0)),
                        operator: BinaryOp::Subtract,
                        right: Box::new(Expr::Integer(5.0))
                    }
                )
            }
        )
    };

    assert_eq!(
        evaluater.eval_expression(&expression3),
        Ok(Value::Boolean(false))
    );

    // 10.0 != (5.0 + (15.0 - 5.0)): true
    let expression4 = Expr::Conditional {
        left: Box::new(Expr::Integer(10.0)),
        operator: ConditionalOp::IsNotEqualTo,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Integer(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Integer(15.0)),
                        operator: BinaryOp::Subtract,
                        right: Box::new(Expr::Integer(5.0))
                    }
                )
            }
        )
    };

    assert_eq!(
        evaluater.eval_expression(&expression4),
        Ok(Value::Boolean(true))
    );

    // 10.0 >= (5.0 + (15.0 - 5.0)): false
    let expression5 = Expr::Conditional {
        left: Box::new(Expr::Integer(10.0)),
        operator: ConditionalOp::IsGreaterOrEqualTo,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Integer(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Integer(15.0)),
                        operator: BinaryOp::Subtract,
                        right: Box::new(Expr::Integer(5.0))
                    }
                )
            }
        )
    };

    assert_eq!(
        evaluater.eval_expression(&expression5),
        Ok(Value::Boolean(false))
    );

    // 10.0 <= (5.0 + (15.0 - 5.0)): true
    let expression6 = Expr::Conditional {
        left: Box::new(Expr::Integer(10.0)),
        operator: ConditionalOp::IsLessOrEqualTo,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Integer(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Integer(15.0)),
                        operator: BinaryOp::Subtract,
                        right: Box::new(Expr::Integer(5.0))
                    }
                )
            }
        )
    };

    assert_eq!(
        evaluater.eval_expression(&expression6),
        Ok(Value::Boolean(true))
    );
    
}

#[test]
fn ok_evaluate_conditional_expression_with_identifiers() {
    let mut environment = Environment::new();

    let _ = environment.define("x".to_string(), Value::Number(10.0));
    let _ = environment.define("y".to_string(), Value::Number(5.0));
    
    let evaluater = Evaluater {environment};

    // x(10.0) == y(5.0): false
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(Expr::Identifier("y".to_string()))
    };

    assert_eq!(
        evaluater.eval_expression(&expression1),
        Ok(Value::Boolean(false))
    );

    // x(10.0) > y(5.0): true
    let expression2 = Expr::Conditional {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: ConditionalOp::IsGreaterThan,
        right: Box::new(Expr::Identifier("y".to_string()))
    };

    assert_eq!(
        evaluater.eval_expression(&expression2),
        Ok(Value::Boolean(true))
    );

    // x(10.0) < y(5.0): false
    let expression3 = Expr::Conditional {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: ConditionalOp::IsLessThan,
        right: Box::new(Expr::Identifier("y".to_string()))
    };

    assert_eq!(
        evaluater.eval_expression(&expression3),
        Ok(Value::Boolean(false))
    );
}

#[test]
fn err_evaluate_conditional_expression() {
    let evaluater = Evaluater::new();

    // "x" == 10.0: Err()
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::String("x".to_string())),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(Expr::Integer(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression1),
        Err(
            EvalError::InvalidConditionalOperation {
                left: Value::String("x".to_string()),
                operator: ConditionalOp::IsEqualTo,
                right: Value::Number(10.0)
            }
        )
    );

    // 10.0 == "x": Err()
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::Integer(10.0)),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(Expr::String("x".to_string())),
    };

    assert_eq!(
        evaluater.eval_expression(&expression1),
        Err(
            EvalError::InvalidConditionalOperation {
                left: Value::Number(10.0),
                operator: ConditionalOp::IsEqualTo,
                right: Value::String("x".to_string()),
            }
        )
    );

    // "x" == "x": Err()
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::String("x".to_string())),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(Expr::String("x".to_string())),
    };

    assert_eq!(
        evaluater.eval_expression(&expression1),
        Err(
            EvalError::InvalidConditionalOperation {
                left: Value::String("x".to_string()),
                operator: ConditionalOp::IsEqualTo,
                right: Value::String("x".to_string())
            }
        )
    );
}

#[test]
fn ok_evaluate_let_statement_to_define_and_access_value_and_test_with_binary_expr_and_conditional() {
    let mut evaluater = Evaluater::new();

    // let x = 10.0 + (10.0 + 10.0): x = 30
    let define_stmt = Stmt::Let {
        identifier: "x".to_string(),
        expression: Expr::Binary {
            left: Box::new(Expr::Integer(10.0)),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Integer(10.0)),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Integer(10.0))
            })
        }
    };

    assert_eq!(
        evaluater.eval_statement(&define_stmt),
        Ok(())
    );

    // let y = 10.0 + (x + 10.0): y = 50
    let access_stmt = Stmt::Let {
        identifier: "y".to_string(),
        expression: Expr::Binary {
            left: Box::new(Expr::Integer(10.0)),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Identifier("x".to_string())),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Integer(10.0))
            })
        }
    };

    assert_eq!(
        evaluater.eval_statement(&access_stmt),
        Ok(())
    );

    // x(30) + y(50): 80.0
    let expression1 = Expr::Binary {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Identifier("y".to_string()))
    };

    assert_eq!(
        evaluater.eval_expression(&expression1),
        Ok(Value::Number(80.0))
    );

    // x(30) < y(50): true
    let expression2 = Expr::Conditional {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: ConditionalOp::IsLessThan,
        right: Box::new(Expr::Identifier("y".to_string()))
    };

    assert_eq!(
        evaluater.eval_expression(&expression2),
        Ok(Value::Boolean(true))
    );
}

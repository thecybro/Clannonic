
use crate::frontend::{
    BinaryOp, BooleanType, ConditionalOp, DataType, Expr, Program, Stmt,
};

use super::{
    Value,
    Environment,

    Function,
    FunctionRegistry,

    Result,
    EvalError,
};

#[derive(Debug)]
pub struct Evaluater {
    environment: Environment,
    function_registry: FunctionRegistry,
}

impl Evaluater {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            function_registry: FunctionRegistry::new(),
        }
    }

    fn get_func(&self, name: &str) -> Option<&Function> {
        self.function_registry.get(name)
    }
    
    pub fn evaluate(
        &mut self,
        program: &Program,
    ) -> Result<()> {
        
        for stmt in program.ast.iter() {
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
                self.eval_while_statement(condition, body)
            },
            Stmt::Fn { name, return_type, return_expr, body } => {
                // Evaluation of function statement registers it
                // 
                // Evaluation of function expression executes it
                self.eval_function_statement(name, return_type, return_expr, body)
            }
        }
    }

    fn eval_let_statement(
        &mut self,
        identifier: &str,
        expression: &Expr
    ) -> Result<Value> {

        let value = self.eval_expression(expression)?;

        self.environment.define(identifier, value)
    }

    fn eval_if_statement(
        &mut self,
        condition: &Expr,
        body: &Box<[Stmt]>
    ) -> Result<()> {

        if self.eval_expression(condition)? == Value::Boolean(BooleanType::True) {
            for stmt in body.iter() {
                self.eval_statement(&stmt)?
            }
        }
        
        Ok(())
    }

    fn eval_while_statement(
        &mut self,
        condition: &Expr,
        body: &Box<[Stmt]>
    ) -> Result<()> {

        while self.eval_expression(condition)? == Value::Boolean(BooleanType::True) {
            for stmt in body.iter() {
                self.eval_statement(&stmt)?
            }
        }

        Ok(())
    }

    /// Evaluating a function statement means registering the
    /// function to the registry, the actual execution
    /// of the function body occurs only when it is called 
    /// 
    /// For example: 
    /// ```
    /// // It's just registered here
    /// fn main() {
    ///     // do something
    /// }
    /// 
    /// let x = main() // the actual execution occurs here
    /// ```
    fn eval_function_statement(
        &mut self,
        name: &str,
        return_type: &Option<DataType>,
        return_expr: &Option<Expr>,
        body: &Box<[Stmt]>
    ) -> Result<()> {
        
        self.function_registry.register(
            name,
            return_type,
            return_expr,
            body
        )?;

        Ok(())
    }
    
    fn eval_expression(
        &mut self,
        expression: &Expr
    ) -> Result<Value> {
        
        match expression {
            Expr::Number(value) => Ok(Value::Number(*value)),
            Expr::String(value) => Ok(Value::String(value.clone())),
            Expr::Identifier(name) => {
                self.environment.get(name).cloned()
            },
            Expr::Boolean(boolean_type) => {
                self.eval_boolean_expression(boolean_type)
            }
            Expr::Binary { left, operator, right } => {
                self.eval_binary_expression(left, operator, right)
            }
            Expr::Conditional { left, operator, right } => {
                self.eval_conditional_expression(left, operator, right)
            },
            Expr::Function { name } => {
                self.eval_function_expression(name)
            }
        }
    }

    fn eval_boolean_expression(
        &mut self,
        boolean_type: &BooleanType
    ) -> Result<Value> {
        match boolean_type {
            BooleanType::True => Ok(Value::Boolean(BooleanType::True)),
            BooleanType::False => Ok(Value::Boolean(BooleanType::False))
        }
    }
    
    fn eval_binary_expression(
        &mut self,
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
        &mut self,
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
                        Ok(Value::Boolean
                            (bool_raw_to_type(left == right))
                        )
                    },
                    ConditionalOp::IsNotEqualTo => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left != right))
                        )
                    },
                    ConditionalOp::IsGreaterThan => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left > right))
                        )
                    },
                    ConditionalOp::IsLessThan => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left < right))
                        )
                    },
                    ConditionalOp::IsGreaterOrEqualTo => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left >= right))
                        )
                    },
                    ConditionalOp::IsLessOrEqualTo => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left <= right))
                        )
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

    /// This is the actual function that evaluates/executes the function
    /// when you call it by doing `let x = func_name()`
    fn eval_function_expression(&mut self, name: &str) -> Result<Value> {
        // Get the function from the registry if it's there
        // 
        // Note that currently, we are returning null
        let Some(function) = self.get_func(name).cloned() else {
            return Ok(Value::Null)
        };

        if let Some(return_type) = function.return_type {
            if let Some(return_expr) = function.return_expr.as_ref() {
                for stmt in function.body.iter() {
                    self.eval_statement(stmt)?;
                }
                let value = self.eval_expression(return_expr)?;

                match value {
                    Value::Number(_) => {
                        if return_type != DataType::Integer {
                            return Err(
                                EvalError::InvalidReturnType(value)
                            )
                        }
                    },
                    Value::String(_) => {
                        if return_type != DataType::String {
                            return Err(
                                EvalError::InvalidReturnType(value)
                            )
                        }
                    },
                    Value::Boolean(_) => {
                        if return_type != DataType::Boolean {
                            return Err(
                                EvalError::InvalidReturnType(value)
                            )
                        }
                    },
                    _ => return Err(
                        EvalError::InvalidReturnType(value)
                    )
                }
            
                return Ok(value)
            } else {
                Ok(Value::Null)
            }
        } else {
            Ok(Value::Null)
        }        

        
    }
}

fn bool_raw_to_type(raw: bool) -> BooleanType {
    match raw {
        true => BooleanType::True,
        false => BooleanType::False
    }
}

#[test]
fn ok_evaluate_binary_expression_with_identifier() {
    // Create a manual environment so we can manually
    // define/insert values into the variable for testing
    let mut environment = Environment::new();
    let function_registry = FunctionRegistry::new();
    
    // Define the variable "x" with 5.0
    let _ = environment.define("x", Value::Number(5.0));
    
    let mut evaluater = Evaluater {
        environment,
        function_registry
    };

    let expression = Expr::Binary {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Number(10.0))
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
    let function_registry = FunctionRegistry::new();
    
    // Define the variable "x" with "a_string"
    let _ = environment.define("x", Value::String("a_string".to_string()));
    
    let mut evaluater = Evaluater {
        environment,
        function_registry
    };

    let expression = Expr::Binary {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Number(10.0))
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
        right: Box::new(Expr::Number(10.0))
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
    let mut evaluater = Evaluater::new();

    // 10.0 + 20.0 = 30.0
    let expression1 = Expr::Binary {
        left: Box::new(Expr::Number(10.0)),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Number(20.0))
    };
    
    assert_eq!(
        evaluater.eval_expression(&expression1),
        Ok(Value::Number(30.0))
    );

    // 20.0 - 10.0 = 10.0
    let expression2 = Expr::Binary {
        left: Box::new(Expr::Number(20.0)),
        operator: BinaryOp::Subtract,
        right: Box::new(Expr::Number(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression2),
        Ok(Value::Number(10.0))
    );

    // 20.0 * 10.0 = 200.0
    let expression3 = Expr::Binary {
        left: Box::new(Expr::Number(20.0)),
        operator: BinaryOp::Multiply,
        right: Box::new(Expr::Number(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression3),
        Ok(Value::Number(200.0))
    );

    // 20.0 / 10.0 = 2.0
    let expression4 = Expr::Binary {
        left: Box::new(Expr::Number(20.0)),
        operator: BinaryOp::Divide,
        right: Box::new(Expr::Number(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression4),
        Ok(Value::Number(2.0))
    );

}

#[test]
fn err_evaluate_divide_by_zero() {
    let mut evaluater = Evaluater::new();

    let expression1 = Expr::Binary{
        left: Box::new(Expr::Number(10.0)),
        operator: BinaryOp::Divide,
        right: Box::new(Expr::Number(0.0))
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
    let mut evaluater = Evaluater::new();

    // "something" + 20.0 = Err()
    let expression1 = Expr::Binary {
        left: Box::new(Expr::String("something".to_string())),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Number(20.0))
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
        right: Box::new(Expr::Number(20.0))
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
        right: Box::new(Expr::Number(20.0))
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
        right: Box::new(Expr::Number(20.0))
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
    let mut evaluater = Evaluater::new();

    let expression = Expr::Binary {
        left: Box::new(Expr::Number(10.0)),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Binary {
            left: Box::new(Expr::Number(20.0)),
            operator: BinaryOp::Multiply,
            right: Box::new(Expr::Number(2.0))
        })
    };

    assert_eq!(
        evaluater.eval_expression(&expression),
        Ok(Value::Number(50.0))
    );
}

#[test]
fn err_evaluate_invalid_binary_expression() {
    let mut evaluater = Evaluater::new();
    
    let expression = Expr::Binary {
        left: Box::new(Expr::Number(10.0)),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Binary {
            left: Box::new(Expr::Number(20.0)),
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
    let mut evaluater = Evaluater::new();

    // 10.0 == 10.0: true
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::Number(10.0)),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(Expr::Number(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression1),
        Ok(Value::Boolean(true))
    );

    // 10.0 > 10.0: false
    let expression2 = Expr::Conditional {
        left: Box::new(Expr::Number(10.0)),
        operator: ConditionalOp::IsGreaterThan,
        right: Box::new(Expr::Number(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression2),
        Ok(Value::Boolean(false))
    );

    // 10.0 < 10.0: false
    let expression3 = Expr::Conditional {
        left: Box::new(Expr::Number(10.0)),
        operator: ConditionalOp::IsLessThan,
        right: Box::new(Expr::Number(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression3),
        Ok(Value::Boolean(false))
    );

    // 10.0 != 10.0: false
    let expression4 = Expr::Conditional {
        left: Box::new(Expr::Number(10.0)),
        operator: ConditionalOp::IsNotEqualTo,
        right: Box::new(Expr::Number(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression4),
        Ok(Value::Boolean(false))
    );

    // 10.0 >= 10.0: true
    let expression5 = Expr::Conditional {
        left: Box::new(Expr::Number(10.0)),
        operator: ConditionalOp::IsGreaterOrEqualTo,
        right: Box::new(Expr::Number(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression5),
        Ok(Value::Boolean(true))
    );

    // 10.0 <= 10.0: true
    let expression6 = Expr::Conditional {
        left: Box::new(Expr::Number(10.0)),
        operator: ConditionalOp::IsLessOrEqualTo,
        right: Box::new(Expr::Number(10.0))
    };

    assert_eq!(
        evaluater.eval_expression(&expression6),
        Ok(Value::Boolean(true))
    );
}

#[test]
fn ok_evaluate_nested_binary_conditional_expression() {
    let mut evaluater = Evaluater::new();

    // 10.0 == (5.0 + 5.0): true
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::Number(10.0)),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Number(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Number(5.0))
            }
        )
    };

    assert_eq!(
        evaluater.eval_expression(&expression1),
        Ok(Value::Boolean(true))
    );

    // 10.0 < (5.0 + 5.0): true
    let expression2 = Expr::Conditional {
        left: Box::new(Expr::Number(10.0)),
        operator: ConditionalOp::IsLessThan,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Number(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Number(5.0))
            }
        )
    };

    assert_eq!(
        evaluater.eval_expression(&expression2),
        Ok(Value::Boolean(false))
    );

    // 10.0 > (5.0 + (15.0 - 5.0)): false
    let expression3 = Expr::Conditional {
        left: Box::new(Expr::Number(10.0)),
        operator: ConditionalOp::IsGreaterThan,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Number(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Number(15.0)),
                        operator: BinaryOp::Subtract,
                        right: Box::new(Expr::Number(5.0))
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
        left: Box::new(Expr::Number(10.0)),
        operator: ConditionalOp::IsNotEqualTo,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Number(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Number(15.0)),
                        operator: BinaryOp::Subtract,
                        right: Box::new(Expr::Number(5.0))
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
        left: Box::new(Expr::Number(10.0)),
        operator: ConditionalOp::IsGreaterOrEqualTo,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Number(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Number(15.0)),
                        operator: BinaryOp::Subtract,
                        right: Box::new(Expr::Number(5.0))
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
        left: Box::new(Expr::Number(10.0)),
        operator: ConditionalOp::IsLessOrEqualTo,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Number(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Number(15.0)),
                        operator: BinaryOp::Subtract,
                        right: Box::new(Expr::Number(5.0))
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
    let function_registry = FunctionRegistry::new();
    
    let _ = environment.define("x", Value::Number(10.0));
    let _ = environment.define("y", Value::Number(5.0));
    
    let mut evaluater = Evaluater {environment, function_registry};

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
    let mut evaluater = Evaluater::new();

    // "x" == 10.0: Err()
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::String("x".to_string())),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(Expr::Number(10.0))
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
        left: Box::new(Expr::Number(10.0)),
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
            left: Box::new(Expr::Number(10.0)),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Number(10.0)),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Number(10.0))
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
            left: Box::new(Expr::Number(10.0)),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Identifier("x".to_string())),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Number(10.0))
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

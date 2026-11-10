
use std::collections::HashMap;
use std::rc::Rc;

use crate::frontend::{
    AssignOp, BinaryOp, BooleanVal, ConditionalOp, DataType, Expr, FunctionParameter, Program, Stmt, UnaryFixity, UnaryOp,
};

use super::{
    Value,
    Environment,
    EnvId,

    Interpreter,

    Function,

    Result,
    EvalError,
};


impl Interpreter {
    pub fn new() -> Self {
        let global_id = EnvId(0);

        let global_env = Environment {
            parent: None,
            values: HashMap::new()
        };

        let mut interpreter = Self {
            environments: vec![global_env],
            globals: global_id,
            current: global_id,
        };

        interpreter.register_builtins();
        interpreter.register_std();

        interpreter
    }

    pub fn evaluate(
        &mut self,
        program: &Program,
    ) -> Result<()> {
        
        for stmt in program.ast.iter() {
            let _ = self.eval_stmt(&stmt)?;
        }

        Ok(())
    }
}

#[allow(unused)]
impl Interpreter {
    fn create_env(&mut self, parent: Option<EnvId>) -> EnvId {
        let id = EnvId(self.environments.len());

        self.environments.push(Environment {
            parent,
            values: HashMap::new()
        });

        id
    }

    /// Register the builtin modules which the user can use directly instead
    /// of having to specify the directory via path::to::module etc
    /// Like println() 
    fn register_builtins(&mut self) {
        // Goes into globals field        
    }

    /// Register the language std
    fn register_std(&mut self) {
        // Goes into globals field
    }

    /// This throws error if the variable already exists on the current environment
    fn define(&mut self, env: EnvId, name: String, value: Value) -> Result<Value> {
        // .get() will give us a value if the value under than identifier already
        // exists, which we dont want to let happen in this define() method
        if self.get(env, &name).is_ok() {
            return Err(
                EvalError::DuplicateVariable(
                    format!("The variable '{name}' already exists")
                )
            )
        }

        // We dont travel to the parent nodes because we want to declare the identifier
        // locally inside the current environment only
        self.environments[env.0]
            .values
            .insert(name.clone(), value.clone());

        println!("Following value has been stored in '{name}'");
        println!("'{value}'\n");
        
        Ok(value)
        
    }

    /// Search the current environment until we keep getting ids
    /// and try getting the value under the name given;
    /// 
    /// If we got one, we return and hence exist from the loop.
    /// 
    /// If we didn't get the required value but the ids are finished,
    /// return None indicating we didn't find any value
    fn get(&self, env: EnvId, name: &str) -> Result<Value> {
        let mut current = Some(env);

        while let Some(id) = current {
            let environment = &self.environments[id.0];

            if let Some(value) = environment.values.get(name) {
                return Ok(value.clone());
            }

            // Since we didn't get value in current scope, we change the
            // current to its parent; so now when the loop starts again,
            // it will get the id of the parent and this will continue until
            // we get the matching identifier or reach global scope
            current = environment.parent;
        }

        // println!("Threw error from interpreter/impl.rs line 135\n");
        // println!("Self is: {self:#?}");
        
        Err(
            EvalError::UndefinedVariable(
                format!("{name}")
            )
        )
    }

    /// Start from the current id and see if the identifier we have gotten is present
    /// or not; if we got one, we assign value to it and return.
    /// 
    /// But if we didn't get that identifier in the current scope, we walk up to its
    /// parents and will keep doing the same until we either get the identifier or
    /// reach global scope
    fn assign(&mut self, env: EnvId, name: &str, value: &Value) -> Result<Value> {
        let mut current = Some(env);

        while let Some(id) = current {
            let environment = &self.environments[id.0];

            if environment.values.contains_key(name) {
                return self.environments[id.0].values.insert(name.to_string(), value.clone())
                    .ok_or_else(
                        || EvalError::FailedToStoreValue(
                            format!("Failed to store '{value}' in '{name}'")
                        )
                    )
            }

            current = environment.parent;
        }

        Err(
            EvalError::UndefinedVariable(
                format!("The identifier '{name}' is undefined!")
            )
        )
    }

    /// Helper for the define method, which will automatically get the current id and call
    /// define()
    fn define_current(&mut self, name: String, value: Value) -> Result<Value> {
        self.define(self.current, name, value)
    }

    fn get_current(&self, name: &str) -> Result<Value> {
        self.get(self.current, name)
    }

    fn assign_current(&mut self, name: &str, value: &Value) -> Result<Value> {
        self.assign(self.current, name, value)
    }

    fn get_parent(&self) -> Option<EnvId> {
        self.environments[self.current.0].parent
    }

    fn exists(&self, env: &EnvId, name: &str) -> bool {
        if self.environments[env.0].values.contains_key(name) {
            return true
        }

        false
    }
    
}

impl Interpreter {
    fn eval_stmt(
        &mut self,
        stmt: &Stmt
    ) -> Result<()> {
        
        match stmt {
            // Let statement declares an identifier and throws an error if it already
            // exists in the current environment
            Stmt::Let { identifier, expression } => {
                let _ = self.eval_let_stmt(identifier.to_string(), expression)?;
                Ok(())
            },
            // Assign statement replaces the identifier's value stored on the available
            // environment, but throws an error if it doesn't exist
            Stmt::Assign { identifier, operator, expression } => {
                let _ = self.eval_assign_stmt(identifier, *operator, expression)?;
                Ok(())
            }
            Stmt::If { condition, body } => {
                self.eval_if_stmt(condition, body)
            },
            Stmt::While { condition, body } => {
                self.eval_while_stmt(condition, body)
            },
            Stmt::Fn { name, parameters, return_type, return_expr, body } => {
                // Evaluation of function statement registers it
                // 
                // Evaluation of function call expression executes it
                self.eval_function_stmt(name, parameters, return_type, return_expr, body)
            },
            Stmt::Return(expr) => {
                let _ = self.eval_expression(expr)?;
                Ok(())
            },
            Stmt::Expression(expr)  => {
                let _ = self.eval_expression(expr)?;
                Ok(())
            }
        }
    }

    fn eval_let_stmt(
        &mut self,
        identifier: String,
        expression: &Expr
    ) -> Result<Value> {

        let value = self.eval_expression(expression)?;

        self.define_current(identifier, value)
    }

    /// Assign value to the stored identifier/value only if it matches the types
    /// If the value being attempted to assign to the stored identifier doesnot match;
    /// we throw an error
    fn eval_assign_stmt(
        &mut self,
        identifier: &str,
        operator: AssignOp,
        expression: &Expr
    ) -> Result<Value> {

        let value = match operator {
            AssignOp::Assign => {self.eval_expression(expression)?}

            AssignOp::AddAssign => {
                let left = Expr::Identifier(identifier.to_string());
                let operator = BinaryOp::Add;

                self.eval_binary_expression(&left, &operator, expression)?
            }

            AssignOp::SubtractAssign => {
                let left = Expr::Identifier(identifier.to_string());
                let operator = BinaryOp::Subtract;

                self.eval_binary_expression(&left, &operator, expression)?
            }
        };

        let stored_val = self.get_current(identifier)?;
        if std::mem::discriminant(&stored_val) == std::mem::discriminant(&value) {
            self.assign_current(identifier, &value)
        } else {
            return Err(EvalError::InvalidAssignOperation {
                identifier: identifier.to_string(),
                operator,
                stored_val,
                expression: expression.clone()
            })
        }
    }

    fn eval_if_stmt(
        &mut self,
        condition: &Expr,
        body: &Box<[Stmt]>
    ) -> Result<()> {

        if self.eval_expression(condition)? == Value::Boolean(BooleanVal::True) {
            for stmt in body.iter() {
                self.eval_stmt(&stmt)?
            }
        }
        
        Ok(())
    }

    fn eval_while_stmt(
        &mut self,
        condition: &Expr,
        body: &Box<[Stmt]>
    ) -> Result<()> {

        while self.eval_expression(condition)? == Value::Boolean(BooleanVal::True) {
            for stmt in body.iter() {
                self.eval_stmt(&stmt)?
            }
        }

        Ok(())
    }

    /// Evaluating a function statement means registering the
    /// function to the registry, the actual execution
    /// of the function body occurs only when it is called 
    /// 
    /// For example: 
    /// ```no_run
    /// // It's just registered here
    /// fn main() {
    ///     // do something
    /// }
    /// 
    /// let x = main() // the actual execution occurs here in this call expression
    /// ```
    fn eval_function_stmt(
        &mut self,
        name: &str,
        parameters: &Box<[FunctionParameter]>,
        return_type: &Option<DataType>,
        return_expr: &Option<Expr>,
        body: &Box<[Stmt]>
    ) -> Result<()> {

        
        // These .clones() can be too expensive too quickly, so we gotta
        // replace it soon
        let function = Function {
                name: name.to_string(),
                parameters: parameters.clone(),
                return_type: return_type.clone(),
                return_expr: return_expr.clone(),
                body: body.clone(),
                closure: self.current
            };

        
        let value = Value::Function(Rc::new(function));
        self.define_current(
            name.to_string(),
            value
        )?;

        // println!("From interpreter/impl.rs line 317, function registry:\n");
        // println!("Just registered function, so interpreter:\n {self:#?}");

        Ok(())
    }
    
    fn eval_expression(
        &mut self,
        expression: &Expr
    ) -> Result<Value> {
        
        match expression {
            Expr::Integer(value) => Ok(Value::Integer(*value)),
            Expr::Float(value) => Ok(Value::Float(*value)),
            Expr::String(value) => Ok(Value::String(value.clone())),
            Expr::Identifier(name) => {
                // println!("From interpreter/impl.rs line 325\n");
                self.get_current(name)
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
            Expr::Unary { operator, operand, fixity } => {
                self.eval_unary_expression(operator, operand, *fixity)
            },
            Expr::FunctionCall { name, arguments } => {
                self.eval_function_call_expression(name, arguments)
            }
        }
    }

    fn eval_boolean_expression(
        &mut self,
        boolean_type: &BooleanVal
    ) -> Result<Value> {
        match boolean_type {
            BooleanVal::True => Ok(Value::Boolean(BooleanVal::True)),
            BooleanVal::False => Ok(Value::Boolean(BooleanVal::False))
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
            (Value::Integer(left), Value::Integer(right)) => {
                match operator {
                    BinaryOp::Add => Ok(Value::Integer(left + right)),
                    BinaryOp::Subtract => Ok(Value::Integer(left - right)),
                    BinaryOp::Multiply => Ok(Value::Integer(left * right)),
                    BinaryOp::Divide => {
                        if right == 0 {
                            return Err(
                                EvalError::InvalidBinaryOperation {
                                    left: Value::Integer(left),
                                    operator: BinaryOp::Divide,
                                    right: Value::Integer(right)
                                }
                            )
                        }

                        Ok(Value::Integer(left / right))
                    },
                }
            },
            (Value::Float(left), Value::Float(right)) => {
                match operator {
                    BinaryOp::Add => Ok(Value::Float(left + right)),
                    BinaryOp::Subtract => Ok(Value::Float(left - right)),
                    BinaryOp::Multiply => Ok(Value::Float(left * right)),
                    BinaryOp::Divide => {
                        if right == 0.0 {
                            return Err(
                                EvalError::InvalidBinaryOperation {
                                    left: Value::Float(left),
                                    operator: BinaryOp::Divide,
                                    right: Value::Float(right)
                                }
                            )
                        }

                        Ok(Value::Float(left / right))
                    },
                }
            },
            (Value::Integer(left), Value::Float(right)) => {
                match operator {
                    BinaryOp::Add => Ok(Value::Float(left as f64 + right)),
                    BinaryOp::Subtract => Ok(Value::Float(left as f64 - right)),
                    BinaryOp::Multiply => Ok(Value::Float(left as f64 * right)),
                    BinaryOp::Divide => {
                        if right == 0.0 {
                            return Err(
                                EvalError::InvalidBinaryOperation {
                                    left: Value::Integer(left),
                                    operator: BinaryOp::Divide,
                                    right: Value::Float(right)
                                }
                            )
                        }

                        Ok(Value::Float(left as f64 / right))
                    },
                }
            },
            (Value::Float(left), Value::Integer(right)) => {
                match operator {
                    BinaryOp::Add => Ok(Value::Float(left + right as f64)),
                    BinaryOp::Subtract => Ok(Value::Float(left - right as f64)),
                    BinaryOp::Multiply => Ok(Value::Float(left * right as f64)),
                    BinaryOp::Divide => {
                        if right == 0 {
                            return Err(
                                EvalError::InvalidBinaryOperation {
                                    left: Value::Float(left),
                                    operator: BinaryOp::Divide,
                                    right: Value::Integer(right)
                                }
                            )
                        }

                        Ok(Value::Float(left / right as f64))
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

    fn eval_unary_expression(
        &mut self,
        operator: &UnaryOp,
        operand: &Box<Expr>,
        fixity: UnaryFixity
    ) -> Result<Value> {
        
        let operand_val = self.eval_expression(operand)?;

        println!("Got operator '{operator:#?}' Operand_val '{operand_val}' and fixity '{fixity:#?}'");
        
        match (operator, operand_val, fixity) {
            (UnaryOp::Plus, Value::Integer(num), UnaryFixity::Prefix) => Ok(Value::Integer(num.abs())),
            (UnaryOp::Plus, Value::Float(num), UnaryFixity::Prefix) => Ok(Value::Float(num.abs())),
            
            (UnaryOp::Minus, Value::Integer(num), UnaryFixity::Prefix) => Ok(Value::Integer(-num)),
            (UnaryOp::Minus, Value::Float(num), UnaryFixity::Prefix) => Ok(Value::Float(-num)),
            
            (UnaryOp::Increment, Value::Integer(num), UnaryFixity::Postfix) => Ok(Value::Integer(num + 1)),
            (UnaryOp::Increment, Value::Float(num), UnaryFixity::Postfix) => Ok(Value::Float(num + 1.0)),
            
            (UnaryOp::Decrement, Value::Integer(num), UnaryFixity::Postfix) => Ok(Value::Integer(num - 1)),
            (UnaryOp::Decrement, Value::Float(num), UnaryFixity::Postfix) => Ok(Value::Float(num - 1.0)),

            (UnaryOp::Not, Value::Boolean(bool_val), UnaryFixity::Prefix) => Ok(Value::Boolean(flip_bool_type(bool_val))),
            
            _ => return Err(EvalError::InvalidUnaryOperation {
                operator: *operator,
                operand: operand.clone(),
                fixity
            })
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
            (Value::Integer(left), Value::Integer(right)) => {
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
            (Value::Integer(left), Value::Float(right)) => {
                match operator {
                    ConditionalOp::IsEqualTo => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left as f64 == right))
                        )
                    },
                    ConditionalOp::IsNotEqualTo => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left as f64 != right))
                        )
                    },
                    ConditionalOp::IsGreaterThan => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left as f64 > right))
                        )
                    },
                    ConditionalOp::IsLessThan => {
                        Ok(Value::Boolean
                            (bool_raw_to_type((left as f64) < right))
                        )
                    },
                    ConditionalOp::IsGreaterOrEqualTo => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left as f64 >= right))
                        )
                    },
                    ConditionalOp::IsLessOrEqualTo => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left as f64 <= right))
                        )
                    }
                }
            },
            (Value::Float(left), Value::Integer(right)) => {
                match operator {
                    ConditionalOp::IsEqualTo => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left == right as f64))
                        )
                    },
                    ConditionalOp::IsNotEqualTo => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left != right as f64))
                        )
                    },
                    ConditionalOp::IsGreaterThan => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left > right as f64))
                        )
                    },
                    ConditionalOp::IsLessThan => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left < right as f64))
                        )
                    },
                    ConditionalOp::IsGreaterOrEqualTo => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left >= right as f64))
                        )
                    },
                    ConditionalOp::IsLessOrEqualTo => {
                        Ok(Value::Boolean
                            (bool_raw_to_type(left <= right as f64))
                        )
                    }
                }
            },
            (Value::Float(left), Value::Float(right)) => {
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
    /// 
    fn eval_function_call_expression(&mut self, name: &str, arguments: &Box<[Expr]>) -> Result<Value> {
        // We would have the entire function body stored in value
        // println!("From interpreter/impl.rs: line 457\n");
        let value = self.get_current(name)?;

        // println!("From interpreter/impl.rs: line 459\nGot {value}");
        
        match value {
            Value::Function(func) => {
                // The environment of the function would have the environment id
                // of the caller and what the function had when it was registered
                // which the closure captures.
                // 
                // Here, the closure is the parent environment of the function,
                // which lets it access the values not passed by the caller but are
                // accessible to it
                let caller_env = self.current;
                let func_call_env = self.create_env(Some(func.closure));

                // We map each call site argument/identifier to the parameter the 
                // called function expects; based on position
                for (parameter, argument) in func.parameters.iter().zip(arguments.iter()) {
                    let arg_val = self.eval_expression(argument)?;
    
                    self.define(
                        // We define the arguments and their values from the callers environment
                        // in the execution environment of the function
                        func_call_env,
                        parameter.name.clone(),
                        arg_val
                    )?;
                }
                
                self.current = func_call_env;

                let result = if let Some(return_type) = func.return_type {
                    if let Some(return_expr) = func.return_expr.as_ref() {
                        for stmt in func.body.iter() {
                            self.eval_stmt(stmt)?;
                        }
                        let value = self.eval_expression(return_expr)?;
        
                        match value {
                            Value::Integer(_) => {
                                if return_type != DataType::Integer {
                                    return Err(
                                        EvalError::InvalidReturnType(value)
                                    )
                                }
                            },
                            Value::Float(_) => {
                                if return_type != DataType::Float {
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
                    
                        value
                    } else {
                        Value::Null
                    }
                } else {
                    Value::Null
                };

                // Set the current execution environment back to the one
                // that the caller had in the time of calling
                self.current = caller_env;
                
                return Ok(result)
            },
            _ => {
                return Err(
                    EvalError::ExpressionNotCallable{
                        error: format!("Given expression '{name}' is not callable"),
                        value,
                    }
                )
            }
        }       
        
    }
}

fn bool_raw_to_type(raw: bool) -> BooleanVal {
    match raw {
        true => BooleanVal::True,
        false => BooleanVal::False
    }
}

fn bool_type_to_raw(bool_type: BooleanVal) -> bool {
    match bool_type {
        BooleanVal::True => true,
        BooleanVal::False => false
    }
}

fn flip_bool_type(boolean: BooleanVal) -> BooleanVal {
    match boolean {
        BooleanVal::True => BooleanVal::False,
        BooleanVal::False => BooleanVal::True
    }
}

#[test]
fn ok_evaluate_binary_expression_with_identifier() {
    // Create a manual environment so we can manually
    // define/insert values into the variable for testing
    let mut interpreter = Interpreter::new();
    
    // Define the variable "x" with 5.0
    let _ = interpreter.define_current("x".to_string(), Value::Integer(5));

    let expression = Expr::Binary {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Integer(10))
    };

    assert_eq!(
        interpreter.eval_expression(&expression),
        Ok(Value::Integer(15))
    );
}

#[test]
fn err_evaluate_binary_expression_with_string_stored_in_identifier() {
    // Create a manual environment so we can manually
    // define/insert values into the variable for testing
    let mut interpreter = Interpreter::new();
    
    // Define the variable "x" with "a_string"
    let _ = interpreter.define_current("x".to_string(), Value::String("a_string".to_string()));

    let expression = Expr::Binary {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Integer(10))
    };

    assert_eq!(
        interpreter.eval_expression(&expression),
        Err(
            EvalError::InvalidBinaryOperation {
                left: Value::String("a_string".to_string()),
                operator: BinaryOp::Add,
                right: Value::Integer(10)
            }
        )
    );

    let expression2 = Expr::Binary {
        left: Box::new(Expr::Identifier("y".to_string())),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Float(10.0))
    };

    assert_eq!(
        interpreter.eval_expression(&expression2),
        Err(
            EvalError::UndefinedVariable("y".to_string())
        )
    );
}

#[test]
fn ok_evaluate_normal_binary_expression() {
    let mut interpreter = Interpreter::new();

    // 10 + 20 = 30
    let expression1 = Expr::Binary {
        left: Box::new(Expr::Integer(10)),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Integer(20))
    };
    
    assert_eq!(
        interpreter.eval_expression(&expression1),
        Ok(Value::Integer(30))
    );

    // 20.0 - 10.0 = 10.0
    let expression2 = Expr::Binary {
        left: Box::new(Expr::Float(20.0)),
        operator: BinaryOp::Subtract,
        right: Box::new(Expr::Float(10.0))
    };

    assert_eq!(
        interpreter.eval_expression(&expression2),
        Ok(Value::Float(10.0))
    );

    // 20 * 10 = 200
    let expression3 = Expr::Binary {
        left: Box::new(Expr::Integer(20)),
        operator: BinaryOp::Multiply,
        right: Box::new(Expr::Integer(10))
    };

    assert_eq!(
        interpreter.eval_expression(&expression3),
        Ok(Value::Integer(200))
    );

    // 20.0 / 10.0 = 2.0
    let expression4 = Expr::Binary {
        left: Box::new(Expr::Float(20.0)),
        operator: BinaryOp::Divide,
        right: Box::new(Expr::Float(10.0))
    };

    assert_eq!(
        interpreter.eval_expression(&expression4),
        Ok(Value::Float(2.0))
    );

}

#[test]
fn err_evaluate_divide_by_zero() {
    let mut interpreter = Interpreter::new();

    let expression1 = Expr::Binary{
        left: Box::new(Expr::Float(10.0)),
        operator: BinaryOp::Divide,
        right: Box::new(Expr::Float(0.0))
    };

    assert_eq!(
        interpreter.eval_expression(&expression1),
        Err(EvalError::InvalidBinaryOperation {
            left: Value::Float(10.0),
            operator: BinaryOp::Divide,
            right: Value::Float(0.0)
        })
    );
}

#[test]
fn err_evaluate_binary_expression_with_string() {
    let mut interpreter = Interpreter::new();

    // "something" + 20.0 = Err()
    let expression1 = Expr::Binary {
        left: Box::new(Expr::String("something".to_string())),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Integer(20))
    };
    
    assert_eq!(
        interpreter.eval_expression(&expression1),
        Err(EvalError::InvalidBinaryOperation {
            left: Value::String("something".to_string()),
            operator: BinaryOp::Add,
            right: Value::Integer(20)
        })
    );

    // "something" - 20.0 = Err()
    let expression2 = Expr::Binary {
        left: Box::new(Expr::String("something".to_string())),
        operator: BinaryOp::Subtract,
        right: Box::new(Expr::Float(20.0))
    };
    
    assert_eq!(
        interpreter.eval_expression(&expression2),
        Err(EvalError::InvalidBinaryOperation {
            left: Value::String("something".to_string()),
            operator: BinaryOp::Subtract,
            right: Value::Float(20.0)
        })
    );

    // "something" * 20.0 = Err()
    let expression3 = Expr::Binary {
        left: Box::new(Expr::String("something".to_string())),
        operator: BinaryOp::Multiply,
        right: Box::new(Expr::Integer(20))
    };
    
    assert_eq!(
        interpreter.eval_expression(&expression3),
        Err(EvalError::InvalidBinaryOperation {
            left: Value::String("something".to_string()),
            operator: BinaryOp::Multiply,
            right: Value::Integer(20)
        })
    );

    // "something" / 20.0 = Err()
    let expression4 = Expr::Binary {
        left: Box::new(Expr::String("something".to_string())),
        operator: BinaryOp::Divide,
        right: Box::new(Expr::Float(20.0))
    };
    
    assert_eq!(
        interpreter.eval_expression(&expression4),
        Err(EvalError::InvalidBinaryOperation {
            left: Value::String("something".to_string()),
            operator: BinaryOp::Divide,
            right: Value::Float(20.0)
        })
    );

    
}

#[test]
fn ok_evaluate_normal_nested_binary_expression() {
    let mut interpreter = Interpreter::new();

    let expression = Expr::Binary {
        left: Box::new(Expr::Integer(10)),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Binary {
            left: Box::new(Expr::Integer(20)),
            operator: BinaryOp::Multiply,
            right: Box::new(Expr::Float(2.0))
        })
    };

    assert_eq!(
        interpreter.eval_expression(&expression),
        Ok(Value::Float(50.0))
    );
}

#[test]
fn err_evaluate_invalid_binary_expression() {
    let mut interpreter = Interpreter::new();
    
    let expression = Expr::Binary {
        left: Box::new(Expr::Float(10.0)),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Binary {
            left: Box::new(Expr::Float(20.0)),
            operator: BinaryOp::Multiply,
            right: Box::new(Expr::String("invalid".to_string()))
        })
    };
    
    assert_eq!(
        interpreter.eval_expression(&expression),
        Err(EvalError::InvalidBinaryOperation {
            left: Value::Float(20.0),
            operator: BinaryOp::Multiply,
            right: Value::String("invalid".to_string())
        })
    )
}

#[test]
fn ok_evaluate_conditional_expression() {
    let mut interpreter = Interpreter::new();

    // 10.0 == 10.0: true
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::Float(10.0)),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(Expr::Float(10.0))
    };

    assert_eq!(
        interpreter.eval_expression(&expression1),
        Ok(Value::Boolean(BooleanVal::True))
    );

    // 10.0 > 10.0: false
    let expression2 = Expr::Conditional {
        left: Box::new(Expr::Float(10.0)),
        operator: ConditionalOp::IsGreaterThan,
        right: Box::new(Expr::Float(10.0))
    };

    assert_eq!(
        interpreter.eval_expression(&expression2),
        Ok(Value::Boolean(BooleanVal::False))
    );

    // 10.0 < 10.0: false
    let expression3 = Expr::Conditional {
        left: Box::new(Expr::Float(10.0)),
        operator: ConditionalOp::IsLessThan,
        right: Box::new(Expr::Float(10.0))
    };

    assert_eq!(
        interpreter.eval_expression(&expression3),
        Ok(Value::Boolean(BooleanVal::False))
    );

    // 10.0 != 10.0: false
    let expression4 = Expr::Conditional {
        left: Box::new(Expr::Float(10.0)),
        operator: ConditionalOp::IsNotEqualTo,
        right: Box::new(Expr::Float(10.0))
    };

    assert_eq!(
        interpreter.eval_expression(&expression4),
        Ok(Value::Boolean(BooleanVal::False))
    );

    // 10.0 >= 10.0: true
    let expression5 = Expr::Conditional {
        left: Box::new(Expr::Float(10.0)),
        operator: ConditionalOp::IsGreaterOrEqualTo,
        right: Box::new(Expr::Float(10.0))
    };

    assert_eq!(
        interpreter.eval_expression(&expression5),
        Ok(Value::Boolean(BooleanVal::True))
    );

    // 10.0 <= 10.0: true
    let expression6 = Expr::Conditional {
        left: Box::new(Expr::Float(10.0)),
        operator: ConditionalOp::IsLessOrEqualTo,
        right: Box::new(Expr::Float(10.0))
    };

    assert_eq!(
        interpreter.eval_expression(&expression6),
        Ok(Value::Boolean(BooleanVal::True))
    );
}

#[test]
fn ok_evaluate_nested_binary_conditional_expression() {
    let mut interpreter = Interpreter::new();

    // 10.0 == (5.0 + 5.0): true
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::Integer(10)),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Integer(5)),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Float(5.0))
            }
        )
    };

    assert_eq!(
        interpreter.eval_expression(&expression1),
        Ok(Value::Boolean(BooleanVal::True))
    );

    // 10.0 < (5.0 + 5.0): true
    let expression2 = Expr::Conditional {
        left: Box::new(Expr::Float(10.0)),
        operator: ConditionalOp::IsLessThan,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Float(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Integer(5))
            }
        )
    };

    assert_eq!(
        interpreter.eval_expression(&expression2),
        Ok(Value::Boolean(BooleanVal::False))
    );

    // 10.0 > (5.0 + (15.0 - 5.0)): false
    let expression3 = Expr::Conditional {
        left: Box::new(Expr::Integer(10)),
        operator: ConditionalOp::IsGreaterThan,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Integer(5)),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Float(15.0)),
                        operator: BinaryOp::Subtract,
                        right: Box::new(Expr::Float(5.0))
                    }
                )
            }
        )
    };

    assert_eq!(
        interpreter.eval_expression(&expression3),
        Ok(Value::Boolean(BooleanVal::False))
    );

    // 10.0 != (5.0 + (15.0 - 5.0)): true
    let expression4 = Expr::Conditional {
        left: Box::new(Expr::Integer(10)),
        operator: ConditionalOp::IsNotEqualTo,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Integer(5)),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Float(15.0)),
                        operator: BinaryOp::Subtract,
                        right: Box::new(Expr::Float(5.0))
                    }
                )
            }
        )
    };

    assert_eq!(
        interpreter.eval_expression(&expression4),
        Ok(Value::Boolean(BooleanVal::True))
    );

    // 10.0 >= (5.0 + (15.0 - 5.0)): false
    let expression5 = Expr::Conditional {
        left: Box::new(Expr::Float(10.0)),
        operator: ConditionalOp::IsGreaterOrEqualTo,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Float(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Integer(15)),
                        operator: BinaryOp::Subtract,
                        right: Box::new(Expr::Integer(5))
                    }
                )
            }
        )
    };

    assert_eq!(
        interpreter.eval_expression(&expression5),
        Ok(Value::Boolean(BooleanVal::False))
    );

    // 10.0 <= (5.0 + (15.0 - 5.0)): true
    let expression6 = Expr::Conditional {
        left: Box::new(Expr::Float(10.0)),
        operator: ConditionalOp::IsLessOrEqualTo,
        right: Box::new(
            Expr::Binary {
                left: Box::new(Expr::Float(5.0)),
                operator: BinaryOp::Add,
                right: Box::new(
                    Expr::Binary {
                        left: Box::new(Expr::Integer(15)),
                        operator: BinaryOp::Subtract,
                        right: Box::new(Expr::Integer(5))
                    }
                )
            }
        )
    };

    assert_eq!(
        interpreter.eval_expression(&expression6),
        Ok(Value::Boolean(BooleanVal::True))
    );
    
}

#[test]
fn ok_evaluate_conditional_expression_with_identifiers() {
    let mut interpreter = Interpreter::new();
    
    let _ = interpreter.define_current("x".to_string(), Value::Float(10.0));
    let _ = interpreter.define_current("y".to_string(), Value::Integer(5));
    
    // x(10.0) == y(5.0): false
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(Expr::Identifier("y".to_string()))
    };

    assert_eq!(
        interpreter.eval_expression(&expression1),
        Ok(Value::Boolean(BooleanVal::False))
    );

    // x(10.0) > y(5.0): true
    let expression2 = Expr::Conditional {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: ConditionalOp::IsGreaterThan,
        right: Box::new(Expr::Identifier("y".to_string()))
    };

    assert_eq!(
        interpreter.eval_expression(&expression2),
        Ok(Value::Boolean(BooleanVal::True))
    );

    // x(10.0) < y(5.0): false
    let expression3 = Expr::Conditional {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: ConditionalOp::IsLessThan,
        right: Box::new(Expr::Identifier("y".to_string()))
    };

    assert_eq!(
        interpreter.eval_expression(&expression3),
        Ok(Value::Boolean(BooleanVal::False))
    );
}

#[test]
fn err_evaluate_conditional_expression() {
    let mut interpreter = Interpreter::new();

    // "x" == 10.0: Err()
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::String("x".to_string())),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(Expr::Float(10.0))
    };

    assert_eq!(
        interpreter.eval_expression(&expression1),
        Err(
            EvalError::InvalidConditionalOperation {
                left: Value::String("x".to_string()),
                operator: ConditionalOp::IsEqualTo,
                right: Value::Float(10.0)
            }
        )
    );

    // 10.0 == "x": Err()
    let expression1 = Expr::Conditional {
        left: Box::new(Expr::Float(10.0)),
        operator: ConditionalOp::IsEqualTo,
        right: Box::new(Expr::String("x".to_string())),
    };

    assert_eq!(
        interpreter.eval_expression(&expression1),
        Err(
            EvalError::InvalidConditionalOperation {
                left: Value::Float(10.0),
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
        interpreter.eval_expression(&expression1),
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
fn ok_evaluate_let_stmt_to_define_and_access_value_and_test_with_binary_expr_and_conditional() {
    let mut interpreter = Interpreter::new();

    // let x = 10.0 + (10.0 + 10.0): x = 30
    let define_stmt = Stmt::Let {
        identifier: "x".to_string(),
        expression: Expr::Binary {
            left: Box::new(Expr::Integer(10)),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Float(10.0)),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Float(10.0))
            })
        }
    };

    assert_eq!(
        interpreter.eval_stmt(&define_stmt),
        Ok(())
    );

    // let y = 10.0 + (x + 10.0): y = 50
    let access_stmt = Stmt::Let {
        identifier: "y".to_string(),
        expression: Expr::Binary {
            left: Box::new(Expr::Integer(10)),
            operator: BinaryOp::Add,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Identifier("x".to_string())),
                operator: BinaryOp::Add,
                right: Box::new(Expr::Float(10.0))
            })
        }
    };

    assert_eq!(
        interpreter.eval_stmt(&access_stmt),
        Ok(())
    );

    // x(30) + y(50): 80.0
    let expression1 = Expr::Binary {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: BinaryOp::Add,
        right: Box::new(Expr::Identifier("y".to_string()))
    };

    assert_eq!(
        interpreter.eval_expression(&expression1),
        Ok(Value::Float(80.0))
    );

    // x(30) < y(50): true
    let expression2 = Expr::Conditional {
        left: Box::new(Expr::Identifier("x".to_string())),
        operator: ConditionalOp::IsLessThan,
        right: Box::new(Expr::Identifier("y".to_string()))
    };

    assert_eq!(
        interpreter.eval_expression(&expression2),
        Ok(Value::Boolean(BooleanVal::True))
    );
}

#[test]
fn ok_and_err_environment_define_and_get() {
    let mut interpreter = Interpreter::new();

    assert_eq!(
        interpreter.define_current(
            "x".to_string(),
            Value::Float(10.0),
        ),
        Ok(Value::Float(10.0)),
    );

    assert_eq!(
        interpreter.define_current(
            "y".to_string(),
            Value::String("something".to_string())
        ),
        Ok(Value::String("something".to_string()))
    );
        
    assert_eq!(
        interpreter.get_current("x"),
        Ok(Value::Float(10.0))
    );

    assert_eq!(
        interpreter.get_current("y"),
        Ok(Value::String("something".to_string()))
    );
    
    assert_eq!(
        interpreter.get_current("missing"),
        Err(EvalError::UndefinedVariable(
            "missing".to_owned()
        ))
    );
}

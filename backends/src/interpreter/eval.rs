use std::collections::{HashMap, hash_map::Entry};
use std::rc::Rc;

use frontends::{ArithmeticOp, AssignOp, ComparisonOp, Type, UnaryFixity, UnaryOp};
use frontends::{BooleanVal, FunctionParameter, TypedExpr, TypedExprKind, TypedProgram, TypedStmt};

use super::{EnvId, Environment, EvalError, Flow, Function, Interpreter, Result, Value};

impl Interpreter {
    pub fn new() -> Self {
        let global_id = EnvId(0);

        let global_env = Environment {
            parent: None,
            values: HashMap::new(),
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

    pub fn evaluate(&mut self, program: &TypedProgram) -> Result<()> {
        // First register all the functions
        // Here, since the current id is the same as globals,
        // the call to define_current() will get you global id
        // So stuff would be saved in global as well
        for stmt in program.ast.iter() {
            let _ = self.eval_function_signatures(&stmt)?;
        }

        for stmt in program.ast.iter() {
            // Evaluate the statement only if it is not a function
            // because we have already registered all functions
            if !matches!(stmt, TypedStmt::Function { .. }) {
                let _ = self.eval_stmt(&stmt)?;
            }
        }

        Ok(())
    }
}

#[allow(unused)]
impl Interpreter {
    /// Take any environment as the parent and create a new environment where
    /// the parent is the env you passed and the values field is where you
    /// push your bindings/identifiers or any value
    ///
    /// The id it gives is simply the number of environments you had before creating
    /// the new one
    ///
    /// How it manages to create unique ids despite not using any fancy hashing techniques
    /// is;
    /// We execute only one statement & expression at a time and when we create a new env
    /// inside those methods; no other method is creating the env id and we simply get the
    /// number of environments we had till that time and make that the id of the current env
    ///
    /// This way, we can even get the index of the environment created, cuz index of the newly
    /// created element is always equal to the length of the Vec we had before creating it
    /// `new_index = old_len`
    fn create_env(&mut self, parent: Option<EnvId>) -> EnvId {
        // Get the total number of environments we have till now
        let id = EnvId(self.environments.len());

        // Make that envid the parent and create a new HashMap as the values
        // of the environment whose parent is the id that was supplied to us
        self.environments.push(Environment {
            parent,
            values: HashMap::new(),
        });

        // The number of enviroments we had when we created a new environment
        // which is the id of the current environment
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

    /// Use this method when we just want to initialize an entry in the given environment
    /// and dont want to overwrite if the key already exists
    ///
    /// This throws error if the variable already exists on the current environment
    fn define(&mut self, env: EnvId, name: String, value: Value) -> Result<Value> {
        // env.0 just extracts the actual usize inside EnvId which has just one tuple field
        match self.environments[env.0].values.entry(name.clone()) {
            // There is no entry with the key/name we got; so we can safely create
            // an entry with the name and value as key/value
            Entry::Vacant(entry) => {
                entry.insert(value.clone());
                println!("Initialized '{name}' with '{value}'");
                Ok(value)
            }
            // The entry already exists; which is an error when trying to initialize
            // an entry, so we throw the error
            Entry::Occupied(_) => {
                return Err(EvalError::DuplicateVariable(format!(
                    "The variable '{name}' already exists"
                )));
            }
        }
    }

    /// Search the current environment until we keep getting ids
    /// and try getting the value under the name given;
    ///
    /// If we got one, we return and hence exist from the loop.
    ///
    /// If we didn't get the required value but the ids are finished,
    /// return None indicating we didn't find any value
    fn get(&self, env: EnvId, name: &str) -> Result<&Value> {
        let mut current = Some(env);

        while let Some(id) = current {
            let environment = &self.environments[id.0];

            if let Some(value) = environment.values.get(name) {
                return Ok(value);
            }

            // Since we didn't get value in current scope, we change the
            // current to its parent; so now when the loop starts again,
            // it will get the id of the parent and this will continue until
            // we get the matching identifier or reach global scope
            current = environment.parent;
        }

        Err(EvalError::UndefinedVariable(format!("{name}")))
    }

    /// Start from the current id and see if the identifier we have gotten is present
    /// or not; if we got one, we assign value to it and return.
    ///
    /// But if we didn't get that identifier in the current scope, we walk up to its
    /// parents and will keep doing the same until we either get the identifier or
    /// reach global scope
    fn assign(&mut self, env: EnvId, name: &str, value: Value) -> Result<Value> {
        let mut current = Some(env);

        // Start from the current environment, if we still have it,
        // which we will have at the first loop.
        //
        // But when we mutate the value of current inside the loop,
        // to move upwards towards the parents in our case, we might
        // eventually reach the point (global scope) where there is no
        // parent; meaning we will get None if we tried to go even upwards
        //
        // So we can say that the identifier we were looking for doesn't exist
        while let Some(id) = current {
            // The current environment is the id we currently have
            let environment = &mut self.environments[id.0];

            // Try getting a mutable reference to the key if it exists;
            //
            // If it exists, we will get Some(val) and we can mutate val
            // with the new value.
            //
            // If it doesn't exist however, we simply ignore it for now
            if let Some(val) = environment.values.get_mut(name) {
                *val = value.clone();
                println!("Assigned '{val}' to '{name}'");
                return Ok(value);
            }

            // If we reached here; it means we didn't get the identifier
            // in the current environment, so move towards the parent of
            // the current environment
            current = environment.parent;
        }

        // If we reached here, it means we didn't find the identifier we were
        // looking for; which is an error when trying to assign value to one
        Err(EvalError::UndefinedVariable(format!(
            "The identifier '{name}' is undefined!"
        )))
    }

    /// Helper for the define method, which will automatically get the current id and call
    /// define()
    fn define_current(&mut self, name: String, value: Value) -> Result<Value> {
        self.define(self.current, name, value)
    }

    fn define_global(&mut self, name: String, value: Value) -> Result<Value> {
        self.define(self.globals, name, value)
    }

    fn get_current(&self, name: &str) -> Result<&Value> {
        self.get(self.current, name)
    }

    fn get_global(&self, name: &str) -> Result<&Value> {
        self.get(self.globals, name)
    }

    fn assign_current(&mut self, name: &str, value: Value) -> Result<Value> {
        self.assign(self.current, name, value)
    }

    fn assign_globals(&mut self, name: &str, value: Value) -> Result<Value> {
        self.assign(self.globals, name, value)
    }

    fn get_parent(&self) -> Option<EnvId> {
        self.environments[self.current.0].parent
    }

    fn exists(&self, env: &EnvId, name: &str) -> bool {
        if self.environments[env.0].values.contains_key(name) {
            return true;
        }

        false
    }
}

impl Interpreter {
    fn eval_stmt(&mut self, stmt: &TypedStmt) -> Result<Flow> {
        match stmt {
            // Let statement declares an identifier and throws an error if it already
            // exists in the current environment
            TypedStmt::Let {
                identifier,
                mutable: _,
                ty: _,
                expression,
            } => {
                self.eval_let_stmt(identifier.to_string(), expression)?;
                Ok(Flow::Continue)
            }
            // Assign statement replaces the identifier's value stored on the available
            // environment, but throws an error if it doesn't exist
            TypedStmt::Assign {
                identifier,
                operator,
                expression,
            } => {
                // println!("Detected assignment statement: \n{identifier} {operator:#?} {expression:#?}");
                self.eval_assign_stmt(identifier, *operator, expression)?;
                Ok(Flow::Continue)
            }
            TypedStmt::If { condition, body } => self.eval_if_stmt(condition, body),
            TypedStmt::While { condition, body } => self.eval_while_stmt(condition, body),
            TypedStmt::Function {
                name,
                parameters,
                return_type,
                body,
            } => {
                // Evaluation of function statement registers it
                //
                // Evaluation of function call expression executes it
                self.eval_function_stmt(name, parameters, *return_type, body)?;
                Ok(Flow::Continue)
            }
            TypedStmt::Return(expression) => {
                let value = self.eval_return_stmt(expression.as_ref())?;
                Ok(Flow::Return(value))
            }
            TypedStmt::Expression(expr) => {
                self.eval_expression(expr)?;
                Ok(Flow::Continue)
            }
            _ => todo!(),
        }
    }

    fn eval_function_signatures(&mut self, stmt: &TypedStmt) -> Result<Flow> {
        match stmt {
            TypedStmt::Function {
                name,
                parameters,
                return_type,
                body,
            } => {
                self.eval_function_stmt(name, parameters, *return_type, body)?;
            }
            _ => {}
        }

        Ok(Flow::Continue)
    }

    fn eval_block(&mut self, body: &[TypedStmt]) -> Result<Flow> {
        for stmt in body {
            match self.eval_stmt(stmt)? {
                Flow::Continue => {}
                Flow::Return(value) => {
                    return Ok(Flow::Return(value));
                }
            }
        }

        Ok(Flow::Continue)
    }

    fn eval_let_stmt(&mut self, identifier: String, expression: &TypedExpr) -> Result<Value> {
        let value = self.eval_expression(expression)?;

        self.define_current(identifier, value)
    }

    fn eval_return_stmt(&mut self, expression: Option<&TypedExpr>) -> Result<Value> {
        if let Some(expr) = expression {
            return self.eval_expression(expr);
        } else {
            Ok(Value::Unit)
        }
    }

    /// Assign value to the stored identifier/value only if it matches the types
    /// If the value being attempted to assign to the stored identifier doesnot match;
    /// we throw an error
    fn eval_assign_stmt(
        &mut self,
        identifier: &str,
        operator: AssignOp,
        expression: &TypedExpr,
    ) -> Result<Value> {
        let value = match operator {
            AssignOp::Assign => self.eval_expression(expression)?,

            AssignOp::AddAssign | AssignOp::SubtractAssign => {
                // let left = TypedExpr::Identifier(identifier.to_string());
                
                let left = TypedExpr {
                    kind: TypedExprKind::Identifier(identifier.to_string()),
                    ty: match self.get_current(identifier)? {
                        Value::Integer(_) => Type::Int,
                        Value::Boolean(_) => Type::Bool,
                        _ => Type::Unit
                    }
                };

                let arithmetic_operator = match operator {
                    AssignOp::AddAssign => ArithmeticOp::Add,
                    AssignOp::SubtractAssign => ArithmeticOp::Subtract,
                    AssignOp::Assign => unreachable!(),
                };

                self.eval_arithmetic_expression(&left, arithmetic_operator, expression)?
            }
        };
        self.assign_current(identifier, value)
    }

    fn eval_if_stmt(&mut self, condition: &TypedExpr, body: &Box<[TypedStmt]>) -> Result<Flow> {
        if self.eval_expression(condition)? == Value::Boolean(BooleanVal::True) {
            self.eval_block(body)
        } else {
            Ok(Flow::Continue)
        }
    }

    fn eval_while_stmt(&mut self, condition: &TypedExpr, body: &Box<[TypedStmt]>) -> Result<Flow> {
        while self.eval_expression(condition)? == Value::Boolean(BooleanVal::True) {
            match self.eval_block(body)? {
                Flow::Continue => {}
                Flow::Return(value) => {
                    return Ok(Flow::Return(value));
                }
            }
        }
        Ok(Flow::Continue)
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
        return_type: Type,
        body: &Box<[TypedStmt]>,
    ) -> Result<Value> {
        // These .clones() can be too expensive too quickly, so we gotta
        // replace it soon
        let function = Function {
            name: name.to_string(),
            parameters: parameters.clone(),
            return_type: return_type.clone(),
            body: body.clone(),
            closure: self.current,
        };

        let value = Value::Function(Rc::new(function));
        self.define_current(name.to_string(), value)?;

        // println!("From interpreter/impl.rs line 317, function registry:\n");
        // println!("Just registered function, so interpreter:\n {self:#?}");

        Ok(Value::Unit)
    }

    fn eval_expression(&mut self, expression: &TypedExpr) -> Result<Value> {
        match &expression.kind {
            TypedExprKind::Integer(value) => Ok(Value::Integer(*value)),
            TypedExprKind::Identifier(name) => {
                Ok(self.get_current(&name)?.clone())
            }
            TypedExprKind::Boolean(boolean_literal) => self.eval_boolean_expression(&boolean_literal),
            TypedExprKind::Arithmetic {
                left,
                operator,
                right,
            } => self.eval_arithmetic_expression(&left, *operator, &right),
            TypedExprKind::Comparison {
                left,
                operator,
                right,
            } => self.eval_comparison_expression(&left, *operator, &right),
            TypedExprKind::Unary {
                operator,
                operand,
                fixity,
            } => self.eval_unary_expression(*operator, &operand, *fixity),
            TypedExprKind::FunctionCall { name, arguments } => {
                self.eval_function_call_expression(&name, &arguments)
            }
            _ => todo!(),
        }
    }

    /// This is the actual function that evaluates/executes the function
    /// when you call it by doing `let x = func_name()`
    ///
    fn eval_function_call_expression(
        &mut self,
        name: &str,
        arguments: &[TypedExpr],
    ) -> Result<Value> {
        // We would have the entire function body stored in value
        // println!("From interpreter/impl.rs: line 457\n");
        let value = self.get_current(name)?.clone();

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
                        arg_val,
                    )?;
                }

                self.current = func_call_env;

                let executed = self.eval_block(&func.body);

                self.current = caller_env;

                let flow = executed?;

                let return_value = match flow {
                    Flow::Return(value) => value,
                    Flow::Continue => Value::Unit,
                };

                Ok(return_value)
            }
            _ => {
                return Err(EvalError::ExpressionNotCallable {
                    error: format!("Given expression '{name}' is not callable"),
                    value: value.clone(),
                });
            }
        }
    }

    fn eval_boolean_expression(&mut self, boolean_type: &BooleanVal) -> Result<Value> {
        match boolean_type {
            BooleanVal::True => Ok(Value::Boolean(BooleanVal::True)),
            BooleanVal::False => Ok(Value::Boolean(BooleanVal::False)),
        }
    }

    fn eval_arithmetic_expression(
        &mut self,
        left: &TypedExpr,
        operator: ArithmeticOp,
        right: &TypedExpr,
    ) -> Result<Value> {
        let left = self.eval_expression(left)?;
        let right = self.eval_expression(right)?;

        match (left, operator, right) {
            (Value::Integer(left), ArithmeticOp::Add, Value::Integer(right)) => {
                Ok(Value::Integer(left + right))
            }
            (Value::Integer(left), ArithmeticOp::Subtract, Value::Integer(right)) => {
                Ok(Value::Integer(left - right))
            }
            (Value::Integer(left), ArithmeticOp::Multiply, Value::Integer(right)) => {
                Ok(Value::Integer(left * right))
            }
            (Value::Integer(left), ArithmeticOp::Divide, Value::Integer(right)) => {
                if right == 0 {
                    return Err(EvalError::InvalidArithmeticOperation {
                        left: Value::Integer(left),
                        operator: ArithmeticOp::Divide,
                        right: Value::Integer(right),
                    });
                }

                Ok(Value::Integer(left / right))
            }
            (left, _, right) => Err(EvalError::InvalidArithmeticOperation {
                left,
                operator,
                right,
            }),
        }
    }

    fn eval_unary_expression(
        &mut self,
        operator: UnaryOp,
        operand: &Box<TypedExpr>,
        fixity: UnaryFixity,
    ) -> Result<Value> {
        let operand_val = self.eval_expression(operand)?;

        match (operator, operand_val, fixity) {
            (UnaryOp::Plus, Value::Integer(num), UnaryFixity::Prefix) => {
                Ok(Value::Integer(num.abs()))
            }

            (UnaryOp::Minus, Value::Integer(num), UnaryFixity::Prefix) => Ok(Value::Integer(-num)),

            (UnaryOp::Increment, Value::Integer(num), UnaryFixity::Postfix) => {
                Ok(Value::Integer(num + 1))
            }

            (UnaryOp::Decrement, Value::Integer(num), UnaryFixity::Postfix) => {
                Ok(Value::Integer(num - 1))
            }

            (UnaryOp::Not, Value::Boolean(bool_val), UnaryFixity::Prefix) => {
                Ok(Value::Boolean(flip_bool_type(bool_val)))
            }

            _ => {
                return Err(EvalError::InvalidUnaryOperation {
                    operator,
                    operand: operand.clone(),
                    fixity,
                });
            }
        }
    }

    fn eval_comparison_expression(
        &mut self,
        left: &TypedExpr,
        operator: ComparisonOp,
        right: &TypedExpr,
    ) -> Result<Value> {
        let left = self.eval_expression(left)?;
        let right = self.eval_expression(right)?;

        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => match operator {
                ComparisonOp::IsEqualTo => Ok(Value::Boolean(bool_raw_to_type(left == right))),
                ComparisonOp::IsNotEqualTo => Ok(Value::Boolean(bool_raw_to_type(left != right))),
                ComparisonOp::IsGreaterThan => Ok(Value::Boolean(bool_raw_to_type(left > right))),
                ComparisonOp::IsLessThan => Ok(Value::Boolean(bool_raw_to_type(left < right))),
                ComparisonOp::IsGreaterOrEqualTo => {
                    Ok(Value::Boolean(bool_raw_to_type(left >= right)))
                }
                ComparisonOp::IsLessOrEqualTo => {
                    Ok(Value::Boolean(bool_raw_to_type(left <= right)))
                }
                _ => todo!(),
            },
            
            (Value::Boolean(left), Value::Boolean(right)) => match operator {
                ComparisonOp::IsEqualTo => Ok(Value::Boolean(bool_raw_to_type(
                    bool_type_to_raw(left) == bool_type_to_raw(right),
                ))),
                ComparisonOp::IsNotEqualTo => Ok(Value::Boolean(bool_raw_to_type(
                    bool_type_to_raw(left) != bool_type_to_raw(right),
                ))),
                ComparisonOp::IsGreaterThan => Ok(Value::Boolean(bool_raw_to_type(
                    bool_type_to_raw(left) > bool_type_to_raw(right),
                ))),
                ComparisonOp::IsLessThan => Ok(Value::Boolean(bool_raw_to_type(
                    bool_type_to_raw(left) < bool_type_to_raw(right),
                ))),
                ComparisonOp::IsGreaterOrEqualTo => Ok(Value::Boolean(bool_raw_to_type(
                    bool_type_to_raw(left) >= bool_type_to_raw(right),
                ))),
                ComparisonOp::IsLessOrEqualTo => Ok(Value::Boolean(bool_raw_to_type(
                    bool_type_to_raw(left) <= bool_type_to_raw(right),
                ))),
                _ => todo!(),
            },
            (left, right) => {
                return Err(EvalError::InvalidComparisonOperation {
                    left,
                    operator,
                    right,
                });
            }
        }
    }
}

fn bool_raw_to_type(raw: bool) -> BooleanVal {
    match raw {
        true => BooleanVal::True,
        false => BooleanVal::False,
    }
}

fn bool_type_to_raw(bool_type: BooleanVal) -> bool {
    match bool_type {
        BooleanVal::True => true,
        BooleanVal::False => false,
    }
}

fn flip_bool_type(boolean: BooleanVal) -> BooleanVal {
    match boolean {
        BooleanVal::True => BooleanVal::False,
        BooleanVal::False => BooleanVal::True,
    }
}


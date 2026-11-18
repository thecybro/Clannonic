use std::collections::HashMap;

use crate::{FunctionParameter, TypedExpr, TypedExprKind, TypedProgram, TypedStmt};
use crate::parse::{RawExpr, RawFunctionParameter, RawProgram, RawStmt};
use crate::{
    Type,
    language::{ArithmeticOp, AssignOp, ComparisonOp, UnaryFixity, UnaryOp},
};

use super::environment::{Binding, TypeEnvironment};
use super::error::{Result, TypeError};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeChecker {
    environment: TypeEnvironment,
    functions: HashMap<String, FunctionSignature>,
    curr_ret_type: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSignature {
    pub parameters: Box<[Type]>,
    pub return_type: Type,
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            environment: TypeEnvironment::new(),
            functions: HashMap::new(),
            curr_ret_type: None,
        }
    }

    pub fn check_program(&mut self, program: RawProgram) -> Result<TypedProgram> {
        for stmt in program.ast.iter() {
            // println!("Collecting function signatures");
            self.collect_function_signature(stmt)?;
        }

        let mut ast = Vec::new();
        for stmt in program.ast.iter() {
            // println!("Checking statements");
            ast.push(self.check_statement(stmt)?);
        }

        Ok(TypedProgram {
            ast: ast.into_boxed_slice()
        })
    }

    fn check_statement(&mut self, statement: &RawStmt) -> Result<TypedStmt> {
        match statement {
            RawStmt::Let {
                identifier,
                mutable,
                annotation,
                expression,
            } => self.check_let_stmt(identifier, *mutable, *annotation, expression),

            RawStmt::Assign {
                identifier,
                operator,
                expression,
            } => self.check_assign_stmt(identifier, *operator, expression),

            RawStmt::If { condition, body } => self.check_if_stmt(condition, body),

            RawStmt::While { condition, body } => self.check_while_stmt(condition, body),

            RawStmt::Function {
                name,
                parameters,
                return_type,
                body,
            } => self.check_function_body(name, parameters, *return_type, body),

            RawStmt::Return(expression) => self.check_return_stmt(expression.as_ref()),

            RawStmt::Expression(expression) => {
                let typed_expr = self.check_expression(expression)?;
                Ok(TypedStmt::Expression(typed_expr))
            }
        }
    }

    fn check_expression(&mut self, expression: &RawExpr) -> Result<TypedExpr> {
        match expression {
            RawExpr::Integer(value) => Ok(TypedExpr {
                kind: TypedExprKind::Integer(*value),
                ty: Type::Int,
            }),

            RawExpr::Boolean(bool_literal) => Ok(TypedExpr {
                kind: TypedExprKind::Boolean(*bool_literal),
                ty: Type::Bool,
            }),
            
            RawExpr::Identifier(name) => {
                let binding = self
                    .environment
                    .lookup(name)
                    .ok_or_else(|| TypeError::UndefinedVariable(name.clone()))?;

                Ok(TypedExpr {
                    kind: TypedExprKind::Identifier(name.clone()),
                    ty: binding.ty
                })
            }

            RawExpr::FunctionCall { name, arguments } => {
                self.check_function_call_expression(name, arguments)
            }

            RawExpr::Arithmetic {
                left,
                operator,
                right,
            } => self.check_arithmetic_expression(left, *operator, right),

            RawExpr::Comparison {
                left,
                operator,
                right,
            } => self.check_comparison_expression(left, *operator, right),

            RawExpr::Unary {
                operator,
                operand,
                fixity,
            } => self.check_unary_expression(*operator, operand, *fixity),

            // Other expressions such as float/string are not yet supported in the
            // current minimal build; even though the interpreter can actually
            // evaluate those
            _ => Err(TypeError::UnsupportedExpression(expression.clone())),
        }
    }
}

impl TypeChecker {
    fn check_block(&mut self, body: &[RawStmt]) -> Result<Box<[TypedStmt]>> {
        self.environment.enter_scope();

        let result = body
            .iter()
            .map(|stmt| self.check_statement(stmt))
            .collect::<Result<Vec<_>>>();

        self.environment.exit_scope();

        result.map(Vec::into_boxed_slice)
    }
    
    fn check_let_stmt(
        &mut self,
        identifier: &str,
        mutable: bool,
        annotation: Option<Type>,
        expression: &RawExpr,
    ) -> Result<TypedStmt> {
        // println!("Got following details: ");
        // println!("Ident: {identifier}\nMutable: {mutable}\nAnnotation: {annotation:#?}\nExpression: {expression:#?}");

        let typed_expr = self.check_expression(expression)?;

        let variable_type = if let Some(annotation) = annotation {
            if annotation != typed_expr.ty {
                return Err(TypeError::TypeMismatch {
                    expected: annotation,
                    found: typed_expr.ty,
                });
            }
            annotation
        } else {
            typed_expr.ty
        };

        // println!("\nInferred following:\n");
        // println!("Type: {inferred_type:#?}\n");

        self.environment.define(
            identifier.to_string(),
            Binding {
                ty: variable_type,
                mutable,
            },
        )?;

        Ok(
            TypedStmt::Let{
                identifier: identifier.to_string(),
                mutable,
                ty: variable_type,
                expression: typed_expr
            }
        )
    }

    fn check_if_stmt(&mut self, expression: &RawExpr, body: &[RawStmt]) -> Result<TypedStmt> {
        let typed_expr = self.check_expression(expression)?;

        if typed_expr.ty != Type::Bool {
            return Err(TypeError::TypeMismatch {
                expected: Type::Bool,
                found: typed_expr.ty,
            });
        }

        let body = self.check_block(body)?;
        
        Ok(TypedStmt::If {
            condition: typed_expr,
            body
        })
    }

    fn check_while_stmt(&mut self, condition: &RawExpr, body: &[RawStmt]) -> Result<TypedStmt> {
        let typed_expr = self.check_expression(condition)?;

        if typed_expr.ty != Type::Bool {
            return Err(TypeError::TypeMismatch {
                expected: Type::Bool,
                found: typed_expr.ty,
            });
        }

        let body = self.check_block(body)?;

        Ok(TypedStmt::While {
            condition: typed_expr,
            body
        })
    }

    fn check_assign_stmt(
        &mut self,
        identifier: &str,
        operator: AssignOp,
        expression: &RawExpr,
    ) -> Result<TypedStmt> {
        let stored_ident = match self.environment.lookup(identifier) {
            Some(name) => name,
            None => return Err(TypeError::UndefinedVariable(identifier.to_string())),
        };

        if !stored_ident.mutable {
            return Err(TypeError::ImmutableAssignment(identifier.to_string()));
        }

        let ident_type = stored_ident.ty;
        let right_typed_expr = self.check_expression(expression)?;

        let right_type = right_typed_expr.ty;
        
        // The operator owns the rule, so we just ask it for return type
        // if the types are valid for assignment operation
        if operator.validates(ident_type, right_type) {
            Ok(TypedStmt::Assign {
                identifier: identifier.to_string(),
                operator,
                expression: right_typed_expr
            })
        } else {
            Err(TypeError::TypeMismatch {
                expected: ident_type,
                found: right_type,
            })
        }
    }

    fn check_function_body(
        &mut self,
        name: &str,
        parameters: &[RawFunctionParameter],
        return_type: Type,
        body: &[RawStmt],
    ) -> Result<TypedStmt> {
        // println!("Checking function body now");
        self.environment.enter_scope();

        let prev_return_type = self.curr_ret_type.replace(return_type);

        // Do the parameter binding and statement checking inside an isolated block
        // so if either fails, we can exit from this block and exit the scope
        // afterwards
        //
        // Which wasn't the case when we were doing the param binding and stmt checking
        // inside the main function scope, which could result in an unexited scope,
        // affecting other methods relying on the scopes
        let mut params = Vec::new();
        let typed_body = (|| {
            for parameter in parameters {
                let param = FunctionParameter {
                    name: parameter.name.clone(),
                    ty: parameter.data_type
                };
                self.environment.define(
                    param.name.clone(),
                    Binding {
                        ty: param.ty,
                        mutable: false,
                    },
                )?;
                params.push(param)
            }

            // We look at whether any return type is declared to be returned by the function
            // and later check if we actually returned anything or not; if the type was declared
            //
            // We dont have to check if the types actually match (if return statement is present)
            // because return statement does that itself
            let requires_return = return_type != Type::Unit;

            let mut saw_return = false;

            let mut typed_statements = Vec::new();
            for statement in body {
                if matches!(statement, RawStmt::Return(_)) {
                    saw_return = true;
                }

                typed_statements.push(self.check_statement(statement)?);
            }

            if requires_return && !saw_return {
                return Err(TypeError::InvalidReturnType {
                    expected: return_type,
                    found: Type::Unit,
                });
            }

            Ok(typed_statements.into_boxed_slice())
        })();

        self.curr_ret_type = prev_return_type;
        self.environment.exit_scope();

        // result
        Ok(TypedStmt::Function {
            name: name.to_string(),
            parameters: params.into_boxed_slice(),
            return_type,
            body: typed_body?
        })
    }

    fn check_return_stmt(&mut self, expression: Option<&RawExpr>) -> Result<TypedStmt> {
        let expected = self.curr_ret_type.ok_or(TypeError::ReturnOutsideFunction)?;

        let (typed_expr, _type) = match expression {
            Some(expr) => {
                // println!("Trying to check the expression we got in return statement");
                let found = self.check_expression(expr)?;

                // This found.clone() might not be needed if we improved the logic
                // a bit
                (Some(found.clone()), found.ty)
            }
            None => {
                (None, Type::Unit)
            }
        };

        if expected != _type {
            return Err(TypeError::InvalidReturnType { expected, found: _type });
        }

        Ok(TypedStmt::Return(typed_expr))
    }

    fn check_function_call_expression(
        &mut self,
        name: &str,
        arguments: &[RawExpr],
    ) -> Result<TypedExpr> {
        let function = self
            .lookup_function(name)
            .ok_or_else(|| TypeError::UndefinedFunction(name.to_string()))?
            .clone();

        if arguments.len() != function.parameters.len() {
            return Err(TypeError::WrongArgumentCount {
                expected: function.parameters.len(),
                found: arguments.len(),
            });
        }

        let mut typed_arguments = Vec::with_capacity(arguments.len());
        
        for (index, (argument, expected_type)) in
            arguments.iter().zip(function.parameters.iter()).enumerate()
        {
            let typed_argument = self.check_expression(argument)?;

            if typed_argument.ty != *expected_type {
                return Err(TypeError::InvalidArgumentType {
                    parameter: index,
                    expected: *expected_type,
                    found: typed_argument.ty,
                });
            }

            typed_arguments.push(typed_argument);
        }

        // Ok(function.return_type)
        Ok(TypedExpr {
            kind: TypedExprKind::FunctionCall {
                name: name.to_string(),
                arguments: typed_arguments.into_boxed_slice(),
            },
            ty: function.return_type,
        })
    }

    fn check_arithmetic_expression(
        &mut self,
        left: &RawExpr,
        operator: ArithmeticOp,
        right: &RawExpr,
    ) -> Result<TypedExpr> {
        let typed_left = self.check_expression(left)?;
        let typed_right = self.check_expression(right)?;

        // The operator owns the rule, so we just call it to know whether or not these
        // types form valid arithmetic operation
        
        let left_ty = typed_left.ty;
        let right_ty = typed_right.ty;

        let result_type = operator
            .result_type(left_ty, right_ty)
            .ok_or(
                TypeError::InvalidArithmeticOperation {
                    left: left_ty,
                    operator,
                    right: right_ty
                }
            )?;

        Ok(TypedExpr {
            kind: TypedExprKind::Arithmetic {
                left: Box::new(typed_left),
                operator,
                right: Box::new(typed_right)
            },
            ty: result_type
        })
    }

    fn check_comparison_expression(
        &mut self,
        left: &RawExpr,
        operator: ComparisonOp,
        right: &RawExpr,
    ) -> Result<TypedExpr> {
        
        let typed_left = self.check_expression(left)?;
        let typed_right = self.check_expression(right)?;

        let left_ty = typed_left.ty;
        let right_ty = typed_right.ty;

        let result_type = operator
            .return_type(left_ty, right_ty)
            .ok_or(
                TypeError::InvalidComparisonOperation {
                    left: left_ty,
                    operator,
                    right: right_ty
                }
            )?;
        
        Ok(TypedExpr {
            kind: TypedExprKind::Comparison {
                left: Box::new(typed_left),
                operator,
                right: Box::new(typed_right)
            },
            ty: result_type
        })
    }

    fn check_unary_expression(
        &mut self,
        operator: UnaryOp,
        operand: &RawExpr,
        fixity: UnaryFixity,
    ) -> Result<TypedExpr> {
        let typed_operand = self.check_expression(operand)?;

        // If the operator is increment or decrement, we need an identifier
        // which is to be incremented/decremented; it doesn't work on integers
        // or other expressions
        // 
        // And to increment/decrement the identifier's value, it needs to be mutable
        // so we check that condition
        if matches!(operator, UnaryOp::Increment | UnaryOp::Decrement) {
            let RawExpr::Identifier(name) = operand else {
                return Err(TypeError::InvalidMutationTarget)
            };

            let binding = self
                .environment
                .lookup(name)
                .ok_or_else(|| TypeError::UndefinedVariable(name.clone()))?;

            if !binding.mutable {
                return Err(TypeError::ImmutableAssignment(name.clone()));
            }
        }
        
        let operand_ty = typed_operand.ty;
        
        let result_type = operator
            .return_type(operand_ty, fixity)
            .ok_or(
                TypeError::InvalidUnaryOperation {
                    operator,
                    found: operand_ty,
                    fixity,
                }
            )?;

        Ok(TypedExpr {
            kind: TypedExprKind::Unary {
                operator,
                operand: Box::new(typed_operand),
                fixity
            },
            ty: result_type
        })
        
    }
}

impl TypeChecker {
    fn define_function(
        &mut self,
        name: String,
        parameters: &[Type],
        return_type: Type,
    ) -> Result<()> {
        if self.functions.contains_key(&name) {
            return Err(TypeError::DuplicateFunctionDeclaration(name));
        }

        self.functions.insert(
            name.clone(),
            FunctionSignature {
                parameters: parameters.into(),
                return_type,
            },
        );

        Ok(())
    }

    fn lookup_function(&self, name: &str) -> Option<&FunctionSignature> {
        self.functions.get(name)
    }

    fn collect_function_signature(&mut self, statement: &RawStmt) -> Result<()> {
        match statement {
            RawStmt::Function {
                name,
                parameters,
                return_type,
                body: _,
            } => {
                let mut params = Vec::new();
                for param in parameters.iter() {
                    params.push(param.data_type)
                }
                self.define_function(name.clone(), &params.into_boxed_slice(), *return_type)
            }
            _ => Ok(()),
        }
    }
}

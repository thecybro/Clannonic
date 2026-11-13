
use std::collections::HashMap;

use crate::{Expr, Program, Stmt, Type, frontend::FunctionParameter};

use super::{Result, TypeError, TypeEnvironment, Binding};


#[derive(Debug, Clone, PartialEq)]
pub struct TypeChecker {
    environment: TypeEnvironment,
    functions: HashMap<String, FunctionSignature>,
    curr_ret_type: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub parameters: Box<[Type]>,
    pub return_type: Type,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            environment: TypeEnvironment::new(),
            functions: HashMap::new(),
            curr_ret_type: None,
        }
    }
    
    pub fn check_program(
        &mut self,
        program: &Program,
    ) -> Result<()> {            
        for stmt in program.ast.iter() {
            println!("Collecting function signatures");
            self.collect_function_signature(stmt)?;
        }
            
        for stmt in program.ast.iter() {
            println!("Checking statements");
            self.check_statement(stmt)?;
        }

        Ok(())
    }
    
    fn check_statement(
        &mut self,
        statement: &Stmt,
    ) -> Result<()> {
        match statement {
            Stmt::Let { 
                identifier, 
                mutable, 
                annotation, 
                expression 
            } => self.check_let_stmt(identifier, *mutable, *annotation, expression),

            Stmt::Function {
                name,
                parameters,
                return_type,
                body,
            } => self.check_function_body(name, parameters, *return_type, body),
            
            Stmt::Return(expression) => self.check_return_stmt(expression),
            
            _ => todo!()
        }
    }

    fn check_expression(
        &mut self,
        expression: &Expr,
    ) -> Result<Type> {
        match expression {
            Expr::Integer(_) => Ok(Type::Int),

            Expr::Boolean(_) => Ok(Type::Bool),

            Expr::Identifier(name) => {
                self.environment
                    .lookup(name)
                    .map(|binding| binding.ty)
                    .ok_or_else(|| TypeError::UndefinedVariable(name.clone()))
            },

            Expr::FunctionCall { name, arguments } => self.check_function_call_expression(name, arguments),
            
            _ => todo!()
        }
    }
}

impl TypeChecker {
    fn check_let_stmt(
        &mut self,
        identifier: &str,
        mutable: bool,
        annotation: Option<Type>,
        expression: &Expr
    ) -> Result<()> {

        println!("Got following details: ");
        println!("Ident: {identifier}\nMutable: {mutable}\nAnnotation: {annotation:#?}\nExpression: {expression:#?}");
        
        let inferred_type = self.check_expression(expression)?;
        
        let variable_type = if let Some(annotation) = annotation {
            if annotation != inferred_type {
                return Err(TypeError::TypeMismatch {
                    expected: inferred_type,
                    found: annotation
                });
            }
            annotation
        } else {
          inferred_type  
        };

        println!("\nInferred following:\n");
        println!("Type: {inferred_type:#?}\n");
        
        self.environment.define(
            identifier.to_string(),
            Binding {
                ty: variable_type,
                mutable
            }
        )?;
        
        Ok(())
    }

    fn check_function_body(
        &mut self,
        name: &str,
        parameters: &Box<[FunctionParameter]>,
        return_type: Type,
        body: &[Stmt]
    ) -> Result<()> {
        println!("Checking function body now");
        self.environment.enter_scope();

        for param in parameters {
            let bin = Binding {
                ty: param.data_type,
                mutable: false,
            };
            let par_name = param.name.clone();
            println!("Binded param '{par_name}' with binding: {bin:#?}");
            self.environment.define(
                param.name.clone(),
                Binding {
                    ty: param.data_type,
                    mutable: false,
                }
            )?
        }

        let prev_ret_type = self.curr_ret_type.replace(return_type);

        for stmt in body {
            println!("Checking following statement present inside function body: \n{stmt:#?}");
            self.check_statement(stmt)?;
        }

        self.curr_ret_type = prev_ret_type;
        self.environment.exit_scope();

        Ok(())
    }

    fn check_return_stmt(
        &mut self,
        expression: &Option<Expr>
    ) -> Result<()> {

        let expected = self.curr_ret_type
            .ok_or(TypeError::ReturnOutsideFunction)?;

        let found = match expression {
            Some(expr) => {
                println!("Trying to check the expression we got in return statement");
                self.check_expression(expr)?
            },
            None => Type::Unit,
        };

        if expected != found {
            return Err(TypeError::InvalidReturnType { 
                expected, 
                found 
            });
        }

        Ok(())
    }

    fn check_function_call_expression(
        &mut self,
        name: &str,
        arguments: &Box<[Expr]>,
    ) -> Result<Type> {

        let function = match self.lookup_function(name) {
            Some(func) => func,
            None => return Err(TypeError::UndefinedFunction(name.to_string()))
        };
        
        Ok(function.return_type)
    }
}

impl TypeChecker {
    fn define_function(
        &mut self,
        name: String,
        parameters: &Box<[Type]>,
        return_type: Type
    ) -> Result<()> {

        if self.functions.contains_key(&name) {
            return Err(TypeError::DuplicateFunctionDeclaration(name))
        }

        let sig = FunctionSignature {
            parameters: parameters.clone(),
            return_type
        } ;
        let def = self.functions.insert(
            name.clone(),
            FunctionSignature {
                parameters: parameters.clone(),
                return_type
            }  
        );

        println!("Just defined the function signature: Name: {name}\n{sig:#?}");

        Ok(())
    }

    fn lookup_function(&self, name: &str) -> Option<&FunctionSignature> {
        self.functions
            .get(name)
    }
    
    fn collect_function_signature(
        &mut self,
        statement: &Stmt
    ) -> Result<()> {
        match statement {
            Stmt::Function {
                name,
                parameters,
                return_type,
                body
            } => {
                let mut params = Vec::new();
                for param in parameters.iter() {
                    params.push(param.data_type)
                }
                self.define_function(
                    name.clone(),
                    &params.into_boxed_slice(),
                    *return_type
                )
            },
            _ => Ok(())
        }
    }
}

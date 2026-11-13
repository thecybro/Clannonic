
use crate::{Type, Program, Stmt, Expr};

use super::{Result, TypeError, TypeEnvironment, Binding};


#[derive(Debug, Clone, PartialEq)]
pub struct TypeChecker {
    environment: TypeEnvironment,
    curr_ret_type: Option<Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            environment: TypeEnvironment::new(),
            curr_ret_type: None
        }
    }
    
    pub fn check_program(
        &mut self,
        program: &Program,
    ) -> Result<()> {
        for stmt in program.ast.iter() {
            self.check_statement(stmt)?;
        }

        Ok(())
    }

    pub fn check_statement(
        &mut self,
        statement: &Stmt,
    ) -> Result<()> {
        match statement {
            Stmt::Let { 
                identifier, 
                mutable, 
                annotation, 
                expression 
            } => {
                self.check_let_stmt(identifier, *mutable, *annotation, expression)
            },
            
            _ => todo!()
        }
    }

    pub fn check_expression(
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
}
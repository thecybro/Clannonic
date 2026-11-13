
use crate::{Type, Stmt, Program};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeEnvironment<'a> {
    program: &'a Program,
    type_checker: Option<TypeChecker>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeChecker {
    stmt: Stmt,
    position: usize
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {}

type Result<T> = std::result::Result<T, TypeError>;

impl<'a> TypeEnvironment<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self {
            program,
            type_checker: None
        }
    }

    pub fn check(&mut self) -> Result<()> {

        println!("All types have been checked!");
        Ok(())
    }
}

mod environment;
mod error;
mod checker;

pub use environment::*;
pub use error::*;
pub use checker::*;

use super::Stmt;

#[derive(Debug, Clone, PartialEq)]
pub struct TypedProgram {
    pub ast: Box<[Stmt]>
}

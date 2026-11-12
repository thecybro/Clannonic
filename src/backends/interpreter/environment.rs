
use std::collections::HashMap;

use super::Value;

#[derive(Debug, Clone)]
pub struct Environment {
    pub(super) parent: Option<EnvId>,
    pub(super) values: HashMap<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnvId(pub usize);

#[derive(Debug)]
pub struct Interpreter {
    pub(super) environments: Vec<Environment>,
    /// Where the top level declarations would be saved.
    /// It includes user defined identifiers, functions
    /// as well as the builtin functions and std modules
    pub(super) globals: EnvId, 
    pub(super) current: EnvId,
}
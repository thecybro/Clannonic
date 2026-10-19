
use std::{
    collections::HashMap,
};

use crate::frontend::{DataType, Expr, Stmt};

use super::{
    Value,

    Result,
    EvalError,
};


#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub(super) name: String,
    pub(super) return_type: Option<DataType>,
    pub(super) return_expr: Option<Expr>,
    pub(super) body: Box<[Stmt]>
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionRegistry {
    functions: HashMap<String, Function>,
}

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new()
        }
    }

    pub fn register(
        &mut self,
        name: &str,
        return_type: &Option<DataType>,
        return_expr: &Option<Expr>,
        body: &Box<[Stmt]>
    ) -> Result<()> {
        let function = Function {
            name: name.to_string(),
            return_type: *return_type,
            return_expr: return_expr.clone(),
            body: body.clone()
        };
        
        // println!("Function '{function:#?}' is about to be stored to registry");

        self.functions.insert(name.to_string(), function);
        
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }
}


impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(
        &mut self,
        name: String,
        value: Value
    ) -> Result<Value> {

        /*
         * I think we should let the user over write the
         * variable??
         * 
         * Maybe there's another solution but since we need
         * to update the value, I am commenting it
         */
        // if self.values.contains_key(&name) {
        //     return Err(
        //         EvalError::DuplicateVariable(
        //             name
        //         )
        //     )
        // }

        self.values.insert(name.clone(), value.clone());

        println!("\nStored '{value:#?}' in '{name:#?}'\n");
        
        Ok(value)
    }

    pub fn get(&self, name: &str) -> Result<&Value> {
        self.values
            .get(name)
            .ok_or_else(|| EvalError::UndefinedVariable(
                name.to_owned()
            ))
    }

}

#[test]
fn ok_and_err_environment_define_and_get() {
    let mut environment = Environment::new();

    assert_eq!(
        environment.define(
            "x".to_owned(),
            Value::Number(10.0),
        ),
        Ok(Value::Number(10.0)),
    );

    assert_eq!(
        environment.define(
            "y".to_owned(),
            Value::String("something".to_string())
        ),
        Ok(Value::String("something".to_string()))
    );
        
    assert_eq!(
        environment.get("x"),
        Ok(&Value::Number(10.0))
    );

    assert_eq!(
        environment.get("y"),
        Ok(&Value::String("something".to_string()))
    );
    
    assert_eq!(
        environment.get("missing"),
        Err(EvalError::UndefinedVariable(
            "missing".to_owned()
        ))
    );
}
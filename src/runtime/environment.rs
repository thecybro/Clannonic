
use std::{
    collections::HashMap,
};

use super::{
    Value,

    Result,
    EvalError,
};


#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Value>,
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

        if self.values.contains_key(&name) {
            return Err(
                EvalError::DuplicateVariable(
                    name
                )
            )
        }

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
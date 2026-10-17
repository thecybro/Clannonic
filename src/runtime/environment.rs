
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
    ) -> Result<()> {

        if self.values.contains_key(&name) {
            return Err(
                EvalError::DuplicateVariable(
                    name
                )
            )
        }

        self.values.insert(name, value.clone());
        Ok(())
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
fn environment() {
    let mut environment = Environment::new();

    assert_eq!(
        environment.define(
            "x".to_owned(),
            Value::Number(10.0),
        ),
        Ok(()),
    );

    assert_eq!(
        environment.define(
            "y".to_owned(),
            Value::String("something".to_string())
        ),
        Ok(())
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

use crate::{
    STACK,
    Store,
};

impl Store {
    pub(crate) fn new() -> Self {
        Self
    }
    
    pub(crate) fn get(&self, variable: &str) -> Option<f64> {
        if let Ok(lock) = STACK.lock() {
            if let Some(value) = lock.get(variable) {
                Some(*value)
            } else {
                None
                // panic!("Couldn't get the value of '{variable}' from the store!");
            }
        } else {
            None
            // panic!("Couldn't get the store to lock to get value from it!");
        }
        
    }

    pub(crate) fn insert(&mut self, variable: &str, value: f64) -> () {
        if let Ok(mut lock) = STACK.lock() {
            lock.insert(variable.to_string(), value);
            // println!("Value of {variable} has been set as {value}");
            ()
        } else {
            panic!("Couldn't set the value of '{variable}' to '{value}'")
        }
    }
}
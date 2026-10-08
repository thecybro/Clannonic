
use crate::{
    Store,
    Evaluater,
    Expression,
    Operator
};

impl Evaluater {
    pub(crate) fn new(expr: Expression) -> Option<f64> {
        
        fn handle_operation(operator: Operator) -> f64 {
            let mut store = Store::new();
            
            match operator {
                Operator::Insert(variable, num) => {
                    store.insert(&variable, num);
                    // println!("Value of {variable} has been set as {num}");
                    num
                }
                Operator::Add(num1, num2, variable) => {
                    let result = num1 + num2;
        
                    if let Some(var) = variable {
                        store.insert(&var, result);
                        // println!("Value of {} is stored as {}", var, result);
                        result
                    } else {
                        result
                    }
                },
                Operator::Subtract(num1, num2, variable) => {
                    let result = num1 - num2;
        
                    if let Some(var) = variable {
                        store.insert(&var, result);
                        // println!("Value of {} is stored as {}", var, result);
                        result
                    } else {
                        result
                    }
                },
                Operator::Multiply(num1, num2, variable) => {
                    let result = num1 * num2;
        
                    if let Some(var) = variable {
                        store.insert(&var, result);
                        // println!("Value of {} is stored as {}", var, result);
                        result
                    } else {
                        result
                    }
                },
                Operator::Divide(num1, num2, variable) => {
                    let result = num1 / num2;
        
                    if let Some(var) = variable {
                        store.insert(&var, result);
                        // println!("Value of {} is stored as {}", var, result);
                        result
                    } else {
                        result
                    }
                },
            }
        }
        
        match expr {
            Expression::Operation(operator) => {
                // Just store the value in the variable or smt
                // Dont return or print it, cuz user hasn't used print
                let _ = handle_operation(operator);
                None
            },
            _ => None
        }
    }
}

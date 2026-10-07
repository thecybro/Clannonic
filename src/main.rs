
use std::{
    io::{
        // self,
        // Write,
        BufRead,
        BufReader,
    },
    fs::File,
};
use std::env;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static STACK: LazyLock<Mutex<HashMap<String, f64>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new()) // start with no elements
});

struct Store;

impl Store {
    fn new() -> Self {
        Self
    }
    
    fn get(&self, variable: &str) -> Option<f64> {
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

    fn insert(&mut self, variable: &str, value: f64) -> () {
        if let Ok(mut lock) = STACK.lock() {
            lock.insert(variable.to_string(), value);
            // println!("Value of {variable} has been set as {value}");
            ()
        } else {
            panic!("Couldn't set the value of '{variable}' to '{value}'")
        }
    }
}

#[derive(Debug)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,

    LeftParen,
    RightParen,
    
    Variable(String),
    Equals,
    Print,

    Eof,
}

#[derive(Debug)]
enum Operator {
    // Option<String> is the variable
    Add(f64, f64, Option<String>),
    Subtract(f64, f64, Option<String>),
    Multiply(f64, f64, Option<String>),
    Divide(f64, f64, Option<String>),
    Insert(String, f64),
}

#[derive(Debug)]
enum Expression {
    Operation(Operator),
    PrintVal(f64),
    PrintComp(Operator),
    Skip,
}

struct Lexer {
    tokens: Vec<Token>
}

struct Parser;

struct Evaluater;

impl Lexer {
    fn new(input: &str) -> Self {
        let mut tokens: Vec<Token> = vec![];
        
        for word in input.split(' ') { // split by whitespace and take whitespace too
            // check if it's a number
            // If not, move to other conditions
            match word.parse::<f64>() {
                Ok(num) => {
                    tokens.push(Token::Number(num));
                    continue
                },
                // Not a number, so lets check for other matches
                Err(_) => match word {
                    " " => {
                        tokens.push(Token::Eof);
                        continue
                    },
                    "" => {
                        tokens.push(Token::Eof);
                        continue
                    },
                    "+" => {
                        tokens.push(Token::Plus);
                        continue
                    },
                    "-" => {
                        tokens.push(Token::Minus);
                        continue
                    },
                    "*" => {
                        tokens.push(Token::Star);
                        continue
                    },
                    "/" => {
                        tokens.push(Token::Slash);
                        continue
                    },
                    "(" => {
                        tokens.push(Token::LeftParen);
                        continue
                    },
                    ")" => {
                        tokens.push(Token::RightParen);
                        continue
                    },
                    "=" => {
                        tokens.push(Token::Equals);
                        continue
                    },
                    "print" => {
                        tokens.push(Token::Print);
                        continue
                    },
                    _ => {
                        tokens.push(Token::Variable(word.to_string()));
                        continue
                    }
                    // _ => panic!("Unrecognized token!")
                }
            }
    
            // If it's not a number and no keyword and identifier we know of, 
        }
    
        Self{
            tokens
        }
    }
}

impl Parser {
    fn new(tokens: &[Token]) -> Expression {
        /*
        * If you want to print an already computed value, like x from x = 1 + 2
        * use print x
        * If you want to calculate the value, then print, use print ( 1 + 2 )
        */
    
        let store = Store::new();
    
        let length_of_tokens = tokens.len();
        // println!("Len of tokens: {length_of_tokens}");
    
        if length_of_tokens <= 0 {
            return Expression::Skip
        }
        match &tokens[0] {
            Token::Variable(var1) => {
                if length_of_tokens <= 1 {
                    panic!("Expected = after {var1}")
                }
                match tokens[1] {
                    Token::Equals => {
                        if length_of_tokens <= 2 {
                            panic!("Expected something after {var1} =")
                        }
                        match &tokens[2] {
                            Token::LeftParen => {
                                if length_of_tokens <= 3{
                                    panic!("A bare '(' found after {var1} = ")
                                }
    
                                match &tokens[3] {
                                    Token::Number(num1) => {
                                        if length_of_tokens <= 4 {
                                            panic!(
                                                "'(' is not properly closed after {var1} = ( {num1}\n 
                                                Or you can also use an operator after it"
                                            )
                                        }
    
                                        match &tokens[4] {
                                            Token::RightParen => {
                                                return Expression::Operation(
                                                    Operator::Insert(var1.clone(), *num1)
                                                )
                                            },
                                            Token::Plus => {
                                                if length_of_tokens <= 5 {
                                                    panic!("Expected a number or variable after {var1} = ( {num1} + ")
                                                }
    
                                                match &tokens[5] {
                                                    Token::Number(num2) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {num1} + {num2}")
                                                        }
    
                                                        match &tokens[6] {
                                                            Token::RightParen => {
                                                                return Expression::Operation(
                                                                    Operator::Add(*num1, *num2, Some(var1.clone()))
                                                                )
                                                            },
                                                            _ => panic!("'(' not properly closed in {var1} = ( {num1} + {num2}")
                                                        }
                                                    },
                                                    Token::Variable(var2) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {num1} + {var2}")
                                                        }
    
                                                        if let Some(var2_val) = store.get(var2) {
                                                            match &tokens[6] {
                                                                Token::RightParen => {
                                                                    return Expression::Operation(
                                                                        Operator::Add(*num1, var2_val, Some(var1.clone()))
                                                                    )
                                                                },
                                                                _ => panic!("'(' not properly closed in {var1} = ( {num1} + {var2}")
                                                            }
                                                        } else {
                                                            panic!("Couldn't get the value of {var2}")
                                                        }
                                                    }
                                                    _ => panic!("Expected a number or variable after {var1} = ( {num1} + ")
                                                }
                                            },
                                            Token::Minus => {
                                                if length_of_tokens <= 5 {
                                                    panic!("Expected a number or variable after {var1} = ( {num1} - ")
                                                }
    
                                                match &tokens[5] {
                                                    Token::Number(num2) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {num1} - {num2}")
                                                        }
    
                                                        match &tokens[6] {
                                                            Token::RightParen => {
                                                                return Expression::Operation(
                                                                    Operator::Subtract(*num1, *num2, Some(var1.clone()))
                                                                )
                                                            },
                                                            _ => panic!("'(' not properly closed in {var1} = ( {num1} - {num2}")
                                                        }
                                                    },
                                                    Token::Variable(var2) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {num1} - {var2}")
                                                        }
    
                                                        if let Some(var2_val) = store.get(var2) {
                                                            match &tokens[6] {
                                                                Token::RightParen => {
                                                                    return Expression::Operation(
                                                                        Operator::Subtract(*num1, var2_val, Some(var1.clone()))
                                                                    )
                                                                },
                                                                _ => panic!("'(' not properly closed in {var1} = ( {num1} - {var2}")
                                                            }
                                                        } else {
                                                            panic!("Couldn't get the value of {var2}")
                                                        }
                                                    }
                                                    _ => panic!("Expected a number or variable after {var1} = ( {num1} - ")
                                                }
                                            },
                                            Token::Star => {
                                                if length_of_tokens <= 5 {
                                                    panic!("Expected a number or variable after {var1} = ( {num1} * ")
                                                }
    
                                                match &tokens[5] {
                                                    Token::Number(num2) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {num1} * {num2}")
                                                        }
    
                                                        match &tokens[6] {
                                                            Token::RightParen => {
                                                                return Expression::Operation(
                                                                    Operator::Multiply(*num1, *num2, Some(var1.clone()))
                                                                )
                                                            },
                                                            _ => panic!("'(' not properly closed in {var1} = ( {num1} * {num2}")
                                                        }
                                                    },
                                                    Token::Variable(var2) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {num1} * {var2}")
                                                        }
    
                                                        if let Some(var2_val) = store.get(var2) {
                                                            match &tokens[6] {
                                                                Token::RightParen => {
                                                                    return Expression::Operation(
                                                                        Operator::Multiply(*num1, var2_val, Some(var1.clone()))
                                                                    )
                                                                },
                                                                _ => panic!("'(' not properly closed in {var1} = ( {num1} * {var2}")
                                                            }
                                                        } else {
                                                            panic!("Couldn't get the value of {var2}")
                                                        }
                                                    }
                                                    _ => panic!("Expected a number or variable after {var1} = ( {num1} * ")
                                                }
                                            },
                                            Token::Slash => {
                                                if length_of_tokens <= 5 {
                                                    panic!("Expected a number or variable after {var1} = ( {num1} / ")
                                                }
    
                                                match &tokens[5] {
                                                    Token::Number(num2) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {num1} / {num2}")
                                                        }
    
                                                        match &tokens[6] {
                                                            Token::RightParen => {
                                                                return Expression::Operation(
                                                                    Operator::Divide(*num1, *num2, Some(var1.clone()))
                                                                )
                                                            },
                                                            _ => panic!("'(' not properly closed in {var1} = ( {num1} / {num2}")
                                                        }
                                                    },
                                                    Token::Variable(var2) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {num1} / {var2}")
                                                        }
    
                                                        if let Some(var2_val) = store.get(var2) {
                                                            match &tokens[6] {
                                                                Token::RightParen => {
                                                                    return Expression::Operation(
                                                                        Operator::Divide(*num1, var2_val, Some(var1.clone()))
                                                                    )
                                                                },
                                                                _ => panic!("'(' not properly closed in {var1} = ( {num1} / {var2}")
                                                            }
                                                        } else {
                                                            panic!("Couldn't get the value of {var2}")
                                                        }
                                                    }
                                                    _ => panic!("Expected a number or variable after {var1} = ( {num1} / ")
                                                }
                                            },
                                            _ => panic!(
                                                "'(' is not properly closed after {var1} = ( {num1}\n 
                                                Or you can also use an operator after it"
                                            )
                                        }
                                    },
                                    Token::Variable(var2) => {
                                        if length_of_tokens <= 4 {
                                            panic!(
                                                "'(' is not properly closed after {var1} = ( {var2}\n 
                                                Or you can also use an operator after it"
                                            )
                                        }
    
                                        if let Some(var2_val) = store.get(var2) {
                                        match &tokens[4] {
                                            Token::RightParen => {
                                                return Expression::Operation(
                                                    Operator::Insert(var1.to_string(), var2_val)
                                                )
                                            },
                                            Token::Plus => {
                                                if length_of_tokens <= 5 {
                                                    panic!("Expected a number or variable after {var1} = ( {var2} + ")
                                                }
    
                                                match &tokens[5] {
                                                    Token::Number(num1) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {var2} + {num1}")
                                                        }
    
                                                        match &tokens[6] {
                                                            Token::RightParen => {
                                                                return Expression::Operation(
                                                                    Operator::Add(*num1, var2_val, Some(var1.clone()))
                                                                )
                                                            },
                                                            _ => panic!("'(' not properly closed in {var1} = ( {num1} + {var2}")
                                                        }
                                                    },
                                                    Token::Variable(var3) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {var2} + {var3}")
                                                        }
    
                                                        if let Some(var3_val) = store.get(var3) {
                                                            match &tokens[6] {
                                                                Token::RightParen => {
                                                                    return Expression::Operation(
                                                                        Operator::Add(var2_val, var3_val, Some(var1.clone()))
                                                                    )
                                                                },
                                                                _ => panic!("'(' not properly closed in {var1} = ( {var2} + {var3}")
                                                            }
                                                        } else {
                                                            panic!("Couldn't get the value of {var3}")
                                                        }
                                                    }
                                                    _ => panic!("Expected a number or variable after {var1} = ( {var2} + ")
                                                }
                                            },
                                            Token::Minus => {
                                                if length_of_tokens <= 5 {
                                                    panic!("Expected a number or variable after {var1} = ( {var2} - ")
                                                }
    
                                                match &tokens[5] {
                                                    Token::Number(num1) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {var2} - {num1}")
                                                        }
    
                                                        match &tokens[6] {
                                                            Token::RightParen => {
                                                                return Expression::Operation(
                                                                    Operator::Subtract(*num1, var2_val, Some(var1.clone()))
                                                                )
                                                            },
                                                            _ => panic!("'(' not properly closed in {var1} = ( {num1} - {var2}")
                                                        }
                                                    },
                                                    Token::Variable(var3) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {var2} - {var3}")
                                                        }
    
                                                        if let Some(var3_val) = store.get(var3) {
                                                            match &tokens[6] {
                                                                Token::RightParen => {
                                                                    return Expression::Operation(
                                                                        Operator::Subtract(var2_val, var3_val, Some(var1.clone()))
                                                                    )
                                                                },
                                                                _ => panic!("'(' not properly closed in {var1} = ( {var2} - {var3}")
                                                            }
                                                        } else {
                                                            panic!("Couldn't get the value of {var3}")
                                                        }
                                                    }
                                                    _ => panic!("Expected a number or variable after {var1} = ( {var2} - ")
                                                }
                                            },
                                            Token::Star => {
                                                if length_of_tokens <= 5 {
                                                    panic!("Expected a number or variable after {var1} = ( {var2} * ")
                                                }
    
                                                match &tokens[5] {
                                                    Token::Number(num1) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {var2} * {num1}")
                                                        }
    
                                                        match &tokens[6] {
                                                            Token::RightParen => {
                                                                return Expression::Operation(
                                                                    Operator::Multiply(*num1, var2_val, Some(var1.clone()))
                                                                )
                                                            },
                                                            _ => panic!("'(' not properly closed in {var1} = ( {num1} * {var2}")
                                                        }
                                                    },
                                                    Token::Variable(var3) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {var2} * {var3}")
                                                        }
    
                                                        if let Some(var3_val) = store.get(var3) {
                                                            match &tokens[6] {
                                                                Token::RightParen => {
                                                                    return Expression::Operation(
                                                                        Operator::Multiply(var2_val, var3_val, Some(var1.clone()))
                                                                    )
                                                                },
                                                                _ => panic!("'(' not properly closed in {var1} = ( {var2} * {var3}")
                                                            }
                                                        } else {
                                                            panic!("Couldn't get the value of {var3}")
                                                        }
                                                    }
                                                    _ => panic!("Expected a number or variable after {var1} = ( {var2} * ")
                                                }
                                            },
                                            Token::Slash => {
                                                if length_of_tokens <= 5 {
                                                    panic!("Expected a number or variable after {var1} = ( {var2} / ")
                                                }
    
                                                match &tokens[5] {
                                                    Token::Number(num1) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {var2} / {num1}")
                                                        }
    
                                                        match &tokens[6] {
                                                            Token::RightParen => {
                                                                // println!("Dividing {var2_val} / {num1}");
                                                                return Expression::Operation(
                                                                    Operator::Divide(var2_val, *num1, Some(var1.clone()))
                                                                )
                                                            },
                                                            _ => panic!("'(' not properly closed in {var1} = ( {num1} / {var2}")
                                                        }
                                                    },
                                                    Token::Variable(var3) => {
                                                        if length_of_tokens <= 6 {
                                                            panic!("'(' not properly closed in {var1} = ( {var2} / {var3}")
                                                        }
    
                                                        if let Some(var3_val) = store.get(var3) {
                                                            match &tokens[6] {
                                                                Token::RightParen => {
                                                                    return Expression::Operation(
                                                                        Operator::Divide(var2_val, var3_val, Some(var1.clone()))
                                                                    )
                                                                },
                                                                _ => panic!("'(' not properly closed in {var1} = ( {var2} / {var3}")
                                                            }
                                                        } else {
                                                            panic!("Couldn't get the value of {var3}")
                                                        }
                                                    }
                                                    _ => panic!("Expected a number or variable after {var1} = ( {var2} / ")
                                                }
                                            },
                                            _ => panic!(
                                                "'(' is not properly closed after {var1} = ( {var2}\n 
                                                Or you can also use an operator after it"
                                            )
                                        }
                                        } else {
                                            panic!("Couldn't get value of {var2}")
                                        }
                                    },
                                    _ => panic!("A bare '(' found after {var1} = ")
                                }
                            },
                            Token::Variable(var2) => {
                                if let Some(var2_val) = store.get(var2) {
                                    if length_of_tokens <= 3 {
                                        return Expression::Operation(
                                            Operator::Insert(var1.clone(), var2_val)
                                        )
                                    }
                                    match tokens[3] {
                                        Token::Eof => {
                                            // condition: x = y
                                            return Expression::Operation(
                                                Operator::Insert(var1.clone(), var2_val)
                                            )
                                        },
                                        Token::Plus => {
                                            if length_of_tokens <= 4 {
                                                panic!("Expected either a variable or number after +")
                                            }
                                            match &tokens[4] {
                                                Token::Variable(var3) => {
                                                    if let Some(var3_val) = store.get(var3) {
                                                        // condition: x = y + z
                                                        return Expression::Operation(
                                                            Operator::Add(
                                                                var2_val, var3_val, Some(var1.clone())
                                                            )
                                                        )
                                                    } else {
                                                        let msg = format!("No value found in variable {var3}");
                                                        panic!("{msg}")
                                                    }
                                                },
                                                Token::Number(num1) => {
                                                    // condition: x = y + 1
                                                    return Expression::Operation(
                                                        Operator::Add(
                                                            var2_val, *num1, Some(var1.clone())
                                                        )
                                                    )
                                                },
                                                _ => panic!("Expected either a number or a variable!")
                                            }
                                        },
                                        Token::Minus => {
                                            if length_of_tokens <= 4 {
                                                panic!("Expected either a variable or number after -")
                                            }
                                            match &tokens[4] {
                                                Token::Variable(var3) => {
                                                    if let Some(var3_val) = store.get(var3) {
                                                        // condition: x = y - z
                                                        return Expression::Operation(
                                                            Operator::Subtract(
                                                                var2_val, var3_val, Some(var1.clone())
                                                            )
                                                        )
                                                    } else {
                                                        let msg = format!("No value found in variable {var3}");
                                                        panic!("{msg}")
                                                    }
                                                },
                                                Token::Number(num1) => {
                                                    // condition: x = y - 1
                                                    return Expression::Operation(Operator::Subtract(var2_val, *num1, Some(var1.clone())))
                                                },
                                                _ => panic!("Expected either a number or a variable!")
                                            }
                                        },
                                        Token::Star => {
                                            if length_of_tokens <= 4 {
                                                panic!("Expected either a variable or number after *")
                                            }
                                            match &tokens[4] {
                                                Token::Variable(var3) => {
                                                    if let Some(var3_val) = store.get(var3) {
                                                        // condition: x = y * z
                                                        return Expression::Operation(
                                                            Operator::Multiply(
                                                                var2_val, var3_val, Some(var1.clone())
                                                            )
                                                        )
                                                    } else {
                                                        let msg = format!("No value found in variable {var3}");
                                                        panic!("{msg}")
                                                    }
                                                },
                                                Token::Number(num1) => {
                                                    // condition: x = y * 1
                                                    return Expression::Operation(Operator::Multiply(var2_val, *num1, Some(var1.clone())))
                                                },
                                                _ => panic!("Expected either a number or a variable!")
                                            }
                                        },
                                        Token::Slash => {
                                            if length_of_tokens <= 4 {
                                                panic!("Expected either a variable or number after /")
                                            }
                                            match &tokens[4] {
                                                Token::Variable(var3) => {
                                                    if let Some(var3_val) = store.get(var3) {
                                                        // condition: x = y / z
                                                        return Expression::Operation(
                                                            Operator::Divide(
                                                                var2_val, var3_val, Some(var1.clone())
                                                            )
                                                        )
                                                    } else {
                                                        let msg = format!("No value found in variable {var3}");
                                                        panic!("{msg}")
                                                    }
                                                },
                                                Token::Number(num1) => {
                                                    // condition: x = y / 1
                                                    return Expression::Operation(Operator::Divide(var2_val, *num1, Some(var1.clone())))
                                                },
                                                _ => panic!("Expected either a number or a variable!")
                                            }
                                        },
                                        _ => panic!("Expected either an operator or nothing!")
                                    }
                                } else {
                                    panic!("No value found in variable {var2}")
                                }
                            },
                            Token::Number(num1) => {
                                if length_of_tokens <= 3 {
                                    return Expression::Operation(
                                        Operator::Insert(var1.to_string(), *num1)
                                    )
                                }
                                match tokens[3] {
                                    Token::Eof => {
                                        // condition: x = 1
                                        return Expression::Operation(
                                            Operator::Insert(var1.clone(), *num1)
                                        )
                                    },
                                    Token::Plus => {
                                        if length_of_tokens <= 4 {
                                            panic!("Expected either a variable or a value after +")
                                        }
                                        match &tokens[4] {
                                            Token::Number(num2) => {
                                                // condition: x = 1 + 2
                                                return Expression::Operation(
                                                    Operator::Add(
                                                        *num1, *num2, Some(var1.clone())
                                                    )
                                                )
                                            },
                                            Token::Variable(var2) => {
                                                // condition: x = 1 + y
                                                if let Some(var2_val) = store.get(var2) {
                                                    return Expression::Operation(
                                                        Operator::Add(
                                                            *num1, var2_val, Some(var1.clone())
                                                        )
                                                    )
                                                } else {
                                                    panic!("No value found in the variable {var2}")
                                                }
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    Token::Minus => {
                                        if length_of_tokens <= 4 {
                                            panic!("Expected either a variable or a value after -")
                                        }
                                        match &tokens[4] {
                                            Token::Number(num2) => {
                                                // condition: x = 1 - 2
                                                return Expression::Operation(
                                                    Operator::Subtract(
                                                        *num1, *num2, Some(var1.clone())
                                                    )
                                                )
                                            },
                                            Token::Variable(var2) => {
                                                // condition: x = 1 - y
                                                if let Some(var2_val) = store.get(var2) {
                                                    return Expression::Operation(
                                                        Operator::Subtract(
                                                            *num1, var2_val, Some(var1.clone())
                                                        )
                                                    )
                                                } else {
                                                    panic!("No value found in the variable {var2}")
                                                }
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    Token::Star => {
                                        if length_of_tokens <= 4 {
                                            panic!("Expected either a variable or a value after *")
                                        }
                                        match &tokens[4] {
                                            Token::Number(num2) => {
                                                // condition: x = 1 * 2
                                                return Expression::Operation(
                                                    Operator::Multiply(
                                                        *num1, *num2, Some(var1.clone())
                                                    )
                                                )
                                            },
                                            Token::Variable(var2) => {
                                                // condition: x = 1 * y
                                                if let Some(var2_val) = store.get(var2) {
                                                    return Expression::Operation(
                                                        Operator::Multiply(
                                                            *num1, var2_val, Some(var1.clone())
                                                        )
                                                    )
                                                } else {
                                                    panic!("No value found in the variable {var2}")
                                                }
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    Token::Slash => {
                                        if length_of_tokens <= 4 {
                                            panic!("Expected either a variable or a value after /")
                                        }
                                        match &tokens[4] {
                                            Token::Number(num2) => {
                                                // condition: x = 1 / 2
                                                return Expression::Operation(
                                                    Operator::Divide(
                                                        *num1, *num2, Some(var1.clone())
                                                    )
                                                )
                                            },
                                            Token::Variable(var2) => {
                                                // condition: x = 1 / y
                                                if let Some(var2_val) = store.get(var2) {
                                                    return Expression::Operation(
                                                        Operator::Divide(
                                                            *num1, var2_val, Some(var1.clone())
                                                        )
                                                    )
                                                } else {
                                                    panic!("No value found in the variable {var2}")
                                                }
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    _ => panic!("Expected either an operator or nothing!")
                                }
                            },
                            _ => panic!("Expected either a variable or a number after {var1} =")
                        }
                    },
                    _ => panic!("Expected = after {var1}")
                }
        },
            Token::Print => {
                if length_of_tokens <= 1 {
                    panic!("Expected either a variable/value or '(' after after print")
                }
                match &tokens[1] {
                    Token::LeftParen => {
                        if length_of_tokens <= 2 {
                            panic!("Expected either a variable or a value after '(' ")
                        }
                        match &tokens[2] {
                            Token::Number(num1) => {
                                if length_of_tokens <= 3 {
                                    panic!("Expected either a ')' or a value after {num1}")
                                }
                                match &tokens[3] {
                                    Token::RightParen => {
                                        return Expression::PrintVal(*num1)
                                    },
                                    Token::Plus => {
                                        if length_of_tokens <= 4 {
                                            panic!("Expected either a variable or a value after +")
                                        }
                                        match &tokens[4] {
                                            Token::Number(num2) => {
                                                return Expression::PrintComp(
                                                    Operator::Add(*num1, *num2, None)
                                                )
                                            },
                                            Token::Variable(var) => {
                                                if let Some(var_val) = store.get(var) {
                                                    return Expression::PrintComp(
                                                        Operator::Add(var_val, *num1, None)
                                                    )
                                                } else {
                                                    panic!("No value stored in variable {var}")
                                                }
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    Token::Minus => {
                                        if length_of_tokens <= 4 {
                                            panic!("Expected either a variable or a value after -")
                                        }
                                        match &tokens[4] {
                                            Token::Number(num2) => {
                                                return Expression::PrintComp(
                                                    Operator::Subtract(*num1, *num2, None)
                                                )
                                            },
                                            Token::Variable(var) => {
                                                if let Some(var_val) = store.get(var) {
                                                    return Expression::PrintComp(
                                                        Operator::Subtract(var_val, *num1, None)
                                                    )
                                                } else {
                                                    panic!("No value stored in variable {var}")
                                                }
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    Token::Star => {
                                        if length_of_tokens <= 4 {
                                            panic!("Expected either a variable or a value after *")
                                        }
                                        match &tokens[4] {
                                            Token::Number(num2) => {
                                                return Expression::PrintComp(
                                                    Operator::Multiply(*num1, *num2, None)
                                                )
                                            },
                                            Token::Variable(var) => {
                                                if let Some(var_val) = store.get(var) {
                                                    return Expression::PrintComp(
                                                        Operator::Multiply(var_val, *num1, None)
                                                    )
                                                } else {
                                                    panic!("No value stored in variable {var}")
                                                }
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    Token::Slash => {
                                        if length_of_tokens <= 4 {
                                            panic!("Expected either a variable or a value after /")
                                        }
                                        match &tokens[4] {
                                            Token::Number(num2) => {
                                                return Expression::PrintComp(
                                                    Operator::Divide(*num1, *num2, None)
                                                )
                                            },
                                            Token::Variable(var) => {
                                                if let Some(var_val) = store.get(var) {
                                                    return Expression::PrintComp(
                                                        Operator::Divide(var_val, *num1, None)
                                                    )
                                                } else {
                                                    panic!("No value stored in variable {var}")
                                                }
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    _ => panic!("Expected either an operator or a ')'")
                                }
                            },
                            Token::Variable(var) => {
                                if let Some(var_val) = store.get(var) {
                                    match &tokens[3] {
                                        Token::RightParen => {
                                            return Expression::PrintVal(var_val)
                                        },
                                        Token::Plus => {
                                            match &tokens[4] {
                                                Token::Number(num1) => {
                                                    match &tokens[5] {
                                                        Token::RightParen => {
                                                            return Expression::PrintComp(
                                                                Operator::Add(var_val, *num1, None)
                                                            )
                                                        },
                                                        _ => panic!("Expected a ) at the end")
                                                    }
                                                },
                                                Token::Variable(var2) => {
                                                    match &tokens[6] {
                                                        Token::RightParen => {
                                                            if let Some(var2_val) = store.get(var2) {
                                                                return Expression::PrintComp(
                                                                    Operator::Add(var_val, var2_val, None)
                                                                )
                                                            } else {
                                                                panic!("The variable {var2} has no value!")
                                                            }
                                                        },
                                                        _ => panic!("Expected a ')' at the end")
                                                    }
                                                },
                                                _ => panic!("Expected either a number or a variable!")
                                            }
                                        },
                                        Token::Minus => {
                                            match &tokens[4] {
                                                Token::Number(num1) => {
                                                    match &tokens[5] {
                                                        Token::RightParen => {
                                                            return Expression::PrintComp(
                                                                Operator::Subtract(var_val, *num1, None)
                                                            )
                                                        },
                                                        _ => panic!("Expected a ) at the end")
                                                    }
                                                },
                                                Token::Variable(var2) => {
                                                    match &tokens[6] {
                                                        Token::RightParen => {
                                                            if let Some(var2_val) = store.get(var2) {
                                                                return Expression::PrintComp(
                                                                    Operator::Subtract(var_val, var2_val, None)
                                                                )
                                                            } else {
                                                                panic!("The variable {var2} has no value!")
                                                            }
                                                        },
                                                        _ => panic!("Expected a ')' at the end")
                                                    }
                                                },
                                                _ => panic!("Expected either a number or a variable!")
                                            }
                                        },
                                        Token::Star => {
                                            match &tokens[4] {
                                                Token::Number(num1) => {
                                                    match &tokens[5] {
                                                        Token::RightParen => {
                                                            return Expression::PrintComp(
                                                                Operator::Multiply(var_val, *num1, None)
                                                            )
                                                        },
                                                        _ => panic!("Expected a ) at the end")
                                                    }
                                                },
                                                Token::Variable(var2) => {
                                                    match &tokens[6] {
                                                        Token::RightParen => {
                                                            if let Some(var2_val) = store.get(var2) {
                                                                return Expression::PrintComp(
                                                                    Operator::Multiply(var_val, var2_val, None)
                                                                )
                                                            } else {
                                                                panic!("The variable {var2} has no value!")
                                                            }
                                                        },
                                                        _ => panic!("Expected a ')' at the end")
                                                    }
                                                },
                                                _ => panic!("Expected either a number or a variable!")
                                            }
                                        },
                                        Token::Slash => {
                                            match &tokens[4] {
                                                Token::Number(num1) => {
                                                    match &tokens[5] {
                                                        Token::RightParen => {
                                                            return Expression::PrintComp(
                                                                Operator::Divide(var_val, *num1, None)
                                                            )
                                                        },
                                                        _ => panic!("Expected a ) at the end")
                                                    }
                                                },
                                                Token::Variable(var2) => {
                                                    match &tokens[6] {
                                                        Token::RightParen => {
                                                            if let Some(var2_val) = store.get(var2) {
                                                                return Expression::PrintComp(
                                                                    Operator::Divide(var_val, var2_val, None)
                                                                )
                                                            } else {
                                                                panic!("The variable {var2} has no value!")
                                                            }
                                                        },
                                                        _ => panic!("Expected a ')' at the end")
                                                    }
                                                },
                                                _ => panic!("Expected either a number or a variable!")
                                            }
                                        },
                                        _ => panic!("Expected either a ')' or a variable/number!")
                                    }
                                } else {
                                    panic!("The variable {var} doesn't have a value!")
                                }
                            },
                            _ => panic!("Expected either a variable or a number!")
                        }
                    },
                    Token::Number(num) => {
                        return Expression::PrintVal(*num)
                    },
                    Token::Variable(var) => {
                        if let Some(var_val) = store.get(var) {
                            return Expression::PrintVal(var_val)
                        } else {
                            panic!("The variable {var} has no value!")
                        }
                    }
                    _ => panic!("Expected either parantheses or a value/variable")
                }
            },
            Token::Eof => {
                return Expression::Skip
            },
            _ => panic!("Expected either a keyword or variable")
        }
    }
}

impl Evaluater {
    fn new(expr: Expression) -> Option<f64> {
        
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
            // Print the value we had stored in the store
            Expression::PrintVal(val) => Some(val),
            Expression::PrintComp(operator) => {
                // compute the value, then return it (cuz of print)
                Some(handle_operation(operator))
            },
            _ => None
        }
    }
}

fn main() {
    // println!("Enter path of the file to analyze");
//     print!(">> ");
//     io::stdout().flush().unwrap();
// 
    let mut input = String::new();
// 
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Couldn't read input!");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        input = args[1].clone();
    } else {
        println!("Clanix interpreter")
    }
    
    let file = File::open(&input.trim()).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        // println!("Line: {}", line);

        let lexed = Lexer::new(&line);
        let parsed = Parser::new(&lexed.tokens);
        if let Some(result) = Evaluater::new(parsed) {
            println!("{}", result)
        }
    }
}

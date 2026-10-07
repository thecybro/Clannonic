
use std::{
    io::{
        self,
        Write,
        BufRead,
        BufReader,
    },
    fs::File,
};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static STACK: LazyLock<Mutex<HashMap<String, i64>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new()) // start with no elements
});

#[derive(Debug)]
enum Token {
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,

    Variable(String),
    Equals,
    Print,

    Eof,
}

#[derive(Debug)]
enum Expression {
    // Option<String> is the variable
    Add(i64, i64, Option<String>),
    Subtract(i64, i64, Option<String>),
    Multiply(i64, i64, Option<String>),
    Divide(i64, i64, Option<String>),
    Insert(String, i64)
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    
    for word in input.split(' ') { // split by whitespace and take whitespace too
        // check if it's a number
        // If not, move to other conditions
        match word.parse::<i64>() {
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
                "=" => {
                    tokens.push(Token::Equals);
                    continue
                },
                "print" => {
                    tokens.push(Token::Print);
                    continue
                },
                _ => {
                    tokens.push(Token::Variable(word.to_string()))
                }
                // _ => panic!("Unrecognized token!")
            }
        }

        // If it's not a number and no keyword and identifier we know of, 
    }

    tokens
}

fn parser(tokens: &[Token]) -> Expression {

    match &tokens[0] {
        Token::Variable(var1) => {
            match tokens[1] {
                Token::Equals => {
                    match &tokens[2] {
                        Token::Variable(var2) => {
                            let lock = STACK.lock().unwrap();
                            if let Some(var2_val) = lock.get(var2) {
                                match tokens[3] {
                                    Token::Eof => {
                                        // condition: x = y
                                        return Expression::Insert(var1.clone(), *var2_val);
                                    },
                                    Token::Plus => {
                                        match &tokens[4] {
                                            Token::Variable(var3) => {
                                                if let Some(var3_val) = lock.get(var3) {
                                                    // condition: x = y + z
                                                    return Expression::Add(*var2_val, *var3_val, Some(var1.clone()))
                                                } else {
                                                    let msg = format!("No value found in variable {var3}");
                                                    panic!("{msg}")
                                                }
                                            },
                                            Token::Number(num1) => {
                                                // condition: x = y + 1
                                                return Expression::Add(*var2_val, *num1, Some(var1.clone()))
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    Token::Minus => {
                                        match &tokens[4] {
                                            Token::Variable(var3) => {
                                                if let Some(var3_val) = lock.get(var3) {
                                                    // condition: x = y - z
                                                    return Expression::Subtract(*var2_val, *var3_val, Some(var1.clone()))
                                                } else {
                                                    let msg = format!("No value found in variable {var3}");
                                                    panic!("{msg}")
                                                }
                                            },
                                            Token::Number(num1) => {
                                                // condition: x = y - 1
                                                return Expression::Subtract(*var2_val, *num1, Some(var1.clone()))
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    Token::Star => {
                                        match &tokens[4] {
                                            Token::Variable(var3) => {
                                                if let Some(var3_val) = lock.get(var3) {
                                                    // condition: x = y * z
                                                    return Expression::Multiply(*var2_val, *var3_val, Some(var1.clone()))
                                                } else {
                                                    let msg = format!("No value found in variable {var3}");
                                                    panic!("{msg}")
                                                }
                                            },
                                            Token::Number(num1) => {
                                                // condition: x = y * 1
                                                return Expression::Multiply(*var2_val, *num1, Some(var1.clone()))
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    Token::Slash => {
                                        match &tokens[4] {
                                            Token::Variable(var3) => {
                                                if let Some(var3_val) = lock.get(var3) {
                                                    // condition: x = y / z
                                                    return Expression::Divide(*var2_val, *var3_val, Some(var1.clone()))
                                                } else {
                                                    let msg = format!("No value found in variable {var3}");
                                                    panic!("{msg}")
                                                }
                                            },
                                            Token::Number(num1) => {
                                                // condition: x = y / 1
                                                return Expression::Divide(*var2_val, *num1, Some(var1.clone()))
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
                            match tokens[3] {
                                Token::Eof => {
                                    // condition: x = 1
                                    return Expression::Insert(var1.clone(), *num1)
                                },
                                Token::Plus => {
                                    match &tokens[4] {
                                        Token::Number(num2) => {
                                            // condition: x = 1 + 2
                                            return Expression::Add(*num1, *num2, Some(var1.clone()))
                                        },
                                        Token::Variable(var2) => {
                                            // condition: x = 1 + y
                                            if let Some(var2_val) = STACK.lock().unwrap().get(var2) {
                                                return Expression::Add(*num1, *var2_val, Some(var1.clone()))
                                            } else {
                                                panic!("No value found in the variable {var2}")
                                            }
                                        },
                                        _ => panic!("Expected either a number or a variable!")
                                    }
                                },
                                Token::Minus => {
                                    match &tokens[4] {
                                        Token::Number(num2) => {
                                            // condition: x = 1 - 2
                                            return Expression::Subtract(*num1, *num2, Some(var1.clone()))
                                        },
                                        Token::Variable(var2) => {
                                            // condition: x = 1 - y
                                            if let Some(var2_val) = STACK.lock().unwrap().get(var2) {
                                                return Expression::Subtract(*num1, *var2_val, Some(var1.clone()))
                                            } else {
                                                panic!("No value found in the variable {var2}")
                                            }
                                        },
                                        _ => panic!("Expected either a number or a variable!")
                                    }
                                },
                                Token::Star => {
                                    match &tokens[4] {
                                        Token::Number(num2) => {
                                            // condition: x = 1 * 2
                                            return Expression::Multiply(*num1, *num2, Some(var1.clone()))
                                        },
                                        Token::Variable(var2) => {
                                            // condition: x = 1 * y
                                            if let Some(var2_val) = STACK.lock().unwrap().get(var2) {
                                                return Expression::Multiply(*num1, *var2_val, Some(var1.clone()))
                                            } else {
                                                panic!("No value found in the variable {var2}")
                                            }
                                        },
                                        _ => panic!("Expected either a number or a variable!")
                                    }
                                },
                                Token::Slash => {
                                    match &tokens[4] {
                                        Token::Number(num2) => {
                                            // condition: x = 1 / 2
                                            return Expression::Divide(*num1, *num2, Some(var1.clone()))
                                        },
                                        Token::Variable(var2) => {
                                            // condition: x = 1 / y
                                            if let Some(var2_val) = STACK.lock().unwrap().get(var2) {
                                                return Expression::Divide(*num1, *var2_val, Some(var1.clone()))
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
                        _ => panic!("Expected either a variable or a number")
                    }
                },
                _ => panic!("Idk")
            }
        },
        _ => panic!("Idk")
    }
}

fn evaluate(expr: Expression) -> i64 {
    match expr {
        Expression::Insert(variable, num) => {
            let mut lock = STACK.lock().unwrap();
            lock.insert(variable, num);
            num
        }
        Expression::Add(num1, num2, variable) => {
            let result = num1 + num2;

            if let Some(var) = variable {
                let mut lock = STACK.lock().unwrap();
                lock.insert(var, result);
                // println!("Value of {} is stored as {}", var, result);
                result
            } else {
                result
            }
        },
        Expression::Subtract(num1, num2, variable) => {
            let result = num1 - num2;

            if let Some(var) = variable {
                let mut lock = STACK.lock().unwrap();
                lock.insert(var, result);
                // println!("Value of {} is stored as {}", var, result);
                result
            } else {
                result
            }
        },
        Expression::Multiply(num1, num2, variable) => {
            let result = num1 * num2;

            if let Some(var) = variable {
                let mut lock = STACK.lock().unwrap();
                lock.insert(var, result);
                // println!("Value of {} is stored as {}", var, result);
                result
            } else {
                result
            }
        },
        Expression::Divide(num1, num2, variable) => {
            let result = num1 / num2;

            if let Some(var) = variable {
                let mut lock = STACK.lock().unwrap();
                lock.insert(var, result);
                // println!("Value of {} is stored as {}", var, result);
                result
            } else {
                result
            }
        },
    }
}

fn main() {
    println!("Enter path of the file to analyze");
    print!(">> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read input!");

    let file = File::open(&input.trim()).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        println!("Line: {}", line);

        let tokens = tokenize(&line);
        println!("Tokens: {:?}", &tokens);
        let parsed = parser(&tokens);
        println!("Parsed: {:?}", &parsed);
        let result = evaluate(parsed);
        
        println!("Result of {} is {}", line, result)
    }
}

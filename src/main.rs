
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

static STACK: LazyLock<Mutex<HashMap<char, i64>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new()) // start with no elements
});

#[derive(PartialEq)]
enum Token {
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,

    Variable(char),
    Equals,
}

enum Expression {
    // Option<char> is the variable
    Add(i64, i64, Option<char>),
    Subtract(i64, i64, Option<char>),
    Multiply(i64, i64, Option<char>),
    Divide(i64, i64, Option<char>)
}

fn tokens(input: &str) -> Vec<Token> {
    let tokens = input
        .chars()
        .filter(|t| !t.is_ascii_whitespace())
        .map(|c| match c {
            '0'..'9' => {
                match c.to_string().parse::<i64>() {
                    Ok(num) => Token::Number(num),
                    Err(_) => panic!("Couldn't parse to numbers!")
                }
            },
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '=' => Token::Equals,
            'a'..'z' | 'A'..'Z' => Token::Variable(c),
            _ => panic!("Unsupported token!")
        }).collect::<Vec<_>>();

    tokens
}

fn parse(tokens: &[Token]) -> Expression {        
    /*
     * Accepted cases:
     * x = 1 + 2 // in this case, interpreter will still tell you the value
     * 1 + 2
     */
    
    // either variable or a number
    match tokens[0] {
        Token::Number(num1) => {
            match tokens[1] {
                Token::Plus => {
                    match tokens[2] {
                        Token::Number(num2) => {
                            return Expression::Add(num1, num2, None)
                        },
                        _ => panic!("Expected a number!")
                    }
                },
                Token::Minus => {
                    match tokens[2] {
                        Token::Number(num2) => {
                            return Expression::Subtract(num1, num2, None)
                        },
                        _ => panic!("Expected a number!")
                    }
                },
                Token::Star => {
                    match tokens[2] {
                        Token::Number(num2) => {
                            return Expression::Multiply(num1, num2, None)
                        },
                        _ => panic!("Expected a number!")
                    }
                },
                Token::Slash => {
                    match tokens[2] {
                        Token::Number(num2) => {
                            return Expression::Divide(num1, num2, None)
                        },
                        _ => panic!("Expected a number!")
                    }
                },
                _ => panic!("Expected an operator!")
            }
        },
        Token::Variable(var) => {
            match tokens[1] {
                Token::Equals => {
                    match tokens[2] {
                        Token::Number(num1) => {
                            match tokens[3] {
                                Token::Plus => {
                                    match tokens[4] {
                                        Token::Number(num2) => {
                                            return Expression::Add(num1, num2, Some(var))
                                        },
                                        _ => panic!("Expected a number!")
                                    }
                                },
                                Token::Minus => {
                                    match tokens[4] {
                                        Token::Number(num2) => {
                                            return Expression::Subtract(num1, num2, Some(var))
                                        },
                                        _ => panic!("Expected a number!")
                                    }
                                },
                                Token::Star => {
                                    match tokens[4] {
                                        Token::Number(num2) => {
                                            return Expression::Multiply(num1, num2, Some(var))
                                        },
                                        _ => panic!("Expected a number!")
                                    }
                                },
                                Token::Slash => {
                                    match tokens[4] {
                                        Token::Number(num2) => {
                                            return Expression::Divide(num1, num2, Some(var))
                                        },
                                        _ => panic!("Expected a number!")
                                    }
                                },
                                _ => panic!("Expected an operator!")
                            }
                        },
                        _ => panic!("Expected a number!"),
                    };
                },
                Token::Plus => {
                    // operator = Some(Token::Plus)
                    match tokens[2] {
                        Token::Number(num) => {
                            /*
                             * If we found like..
                             * x + 2
                             * See if there is any value stored in x and if yes, 
                             * return add expression with that value and 2
                             */
                            let lock = STACK.lock().unwrap();
                            if let Some(value) = lock.get(&var){
                                return Expression::Add(*value, num, None)
                            } else {
                                let msg = format!("The variable {var} doesn't have a value!");
                                panic!("{msg}")
                            }
                        },
                        _ => panic!("Expected a number!")
                    }
                },
                Token::Minus => {
                    // operator = Some(Token::Plus)
                    match tokens[2] {
                        Token::Number(num) => {
                            /*
                             * If we found like..
                             * x - 2
                             * See if there is any value stored in x and if yes, 
                             * return subtract expression with that value and 2
                             */
                            let lock = STACK.lock().unwrap();
                            if let Some(value) = lock.get(&var){
                                return Expression::Subtract(*value, num, None)
                            } else {
                                let msg = format!("The variable {var} doesn't have a value!");
                                panic!("{msg}")
                            }
                        },
                        _ => panic!("Expected a number!")
                    }
                },
                Token::Star => {
                    // operator = Some(Token::Plus)
                    match tokens[2] {
                        Token::Number(num) => {
                            /*
                             * If we found like..
                             * x * 2
                             * See if there is any value stored in x and if yes, 
                             * return multiply expression with that value and 2
                             */
                            let lock = STACK.lock().unwrap();
                            if let Some(value) = lock.get(&var){
                                return Expression::Multiply(*value, num, None)
                            } else {
                                let msg = format!("The variable {var} doesn't have a value!");
                                panic!("{msg}")
                            }
                        },
                        _ => panic!("Expected a number!")
                    }
                },
                Token::Slash => {
                    // operator = Some(Token::Plus)
                    match tokens[2] {
                        Token::Number(num) => {
                            /*
                             * If we found like..
                             * x / 2
                             * See if there is any value stored in x and if yes, 
                             * return divide expression with that value and 2
                             */
                            let lock = STACK.lock().unwrap();
                            if let Some(value) = lock.get(&var){
                                return Expression::Divide(*value, num, None)
                            } else {
                                let msg = format!("The variable {var} doesn't have a value!");
                                panic!("{msg}")
                            }
                        },
                        _ => panic!("Expected a number!")
                    }
                },
                _ => panic!("No operator given after variable declaration!")
            }
    
        },
        _ => panic!("Expected a number or a variable!")
    }
}

fn evaluate(expr: Expression) -> i64 {
    match expr {
        Expression::Add(num1, num2, variable) => {
            let result = num1 + num2;

            if let Some(var) = variable {
                let mut lock = STACK.lock().unwrap();
                lock.insert(var, result);
                println!("Value of {} is stored as {}", var, result);
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
        
        let tokens = tokens(line.as_str());
        let expression = parse(&tokens);
        let result = evaluate(expression);
        println!("Result of {} is {}", line, result)
    }
}

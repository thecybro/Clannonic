
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
    Add(i64, i64, Option<String>),
    Subtract(i64, i64, Option<String>),
    Multiply(i64, i64, Option<String>),
    Divide(i64, i64, Option<String>),
    Insert(String, i64),
}

#[derive(Debug)]
enum Expression {
    Operation(Operator),
    PrintVal(i64),
    PrintComp(Operator)
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
    /*
     * If you want to print an already computed value, like x from x = 1 + 2
     * use print x
     * If you want to calculate the value, then print, use print ( 1 + 2 )
     */
    
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
                                        return Expression::Operation(Operator::Insert(var1.clone(), *var2_val))
                                    },
                                    Token::Plus => {
                                        match &tokens[4] {
                                            Token::Variable(var3) => {
                                                if let Some(var3_val) = lock.get(var3) {
                                                    // condition: x = y + z
                                                    return Expression::Operation(Operator::Add(*var2_val, *var3_val, Some(var1.clone())))
                                                } else {
                                                    let msg = format!("No value found in variable {var3}");
                                                    panic!("{msg}")
                                                }
                                            },
                                            Token::Number(num1) => {
                                                // condition: x = y + 1
                                                return Expression::Operation(Operator::Add(*var2_val, *num1, Some(var1.clone())))
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    Token::Minus => {
                                        match &tokens[4] {
                                            Token::Variable(var3) => {
                                                if let Some(var3_val) = lock.get(var3) {
                                                    // condition: x = y - z
                                                    return Expression::Operation(Operator::Subtract(*var2_val, *var3_val, Some(var1.clone())))
                                                } else {
                                                    let msg = format!("No value found in variable {var3}");
                                                    panic!("{msg}")
                                                }
                                            },
                                            Token::Number(num1) => {
                                                // condition: x = y - 1
                                                return Expression::Operation(Operator::Subtract(*var2_val, *num1, Some(var1.clone())))
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    Token::Star => {
                                        match &tokens[4] {
                                            Token::Variable(var3) => {
                                                if let Some(var3_val) = lock.get(var3) {
                                                    // condition: x = y * z
                                                    return Expression::Operation(Operator::Multiply(*var2_val, *var3_val, Some(var1.clone())))
                                                } else {
                                                    let msg = format!("No value found in variable {var3}");
                                                    panic!("{msg}")
                                                }
                                            },
                                            Token::Number(num1) => {
                                                // condition: x = y * 1
                                                return Expression::Operation(Operator::Multiply(*var2_val, *num1, Some(var1.clone())))
                                            },
                                            _ => panic!("Expected either a number or a variable!")
                                        }
                                    },
                                    Token::Slash => {
                                        match &tokens[4] {
                                            Token::Variable(var3) => {
                                                if let Some(var3_val) = lock.get(var3) {
                                                    // condition: x = y / z
                                                    return Expression::Operation(Operator::Divide(*var2_val, *var3_val, Some(var1.clone())))
                                                } else {
                                                    let msg = format!("No value found in variable {var3}");
                                                    panic!("{msg}")
                                                }
                                            },
                                            Token::Number(num1) => {
                                                // condition: x = y / 1
                                                return Expression::Operation(Operator::Divide(*var2_val, *num1, Some(var1.clone())))
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
                                    return Expression::Operation(Operator::Insert(var1.clone(), *num1))
                                },
                                Token::Plus => {
                                    match &tokens[4] {
                                        Token::Number(num2) => {
                                            // condition: x = 1 + 2
                                            return Expression::Operation(Operator::Add(*num1, *num2, Some(var1.clone())))
                                        },
                                        Token::Variable(var2) => {
                                            // condition: x = 1 + y
                                            if let Some(var2_val) = STACK.lock().unwrap().get(var2) {
                                                return Expression::Operation(Operator::Add(*num1, *var2_val, Some(var1.clone())))
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
                                            return Expression::Operation(Operator::Subtract(*num1, *num2, Some(var1.clone())))
                                        },
                                        Token::Variable(var2) => {
                                            // condition: x = 1 - y
                                            if let Some(var2_val) = STACK.lock().unwrap().get(var2) {
                                                return Expression::Operation(Operator::Subtract(*num1, *var2_val, Some(var1.clone())))
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
                                            return Expression::Operation(Operator::Multiply(*num1, *num2, Some(var1.clone())))
                                        },
                                        Token::Variable(var2) => {
                                            // condition: x = 1 * y
                                            if let Some(var2_val) = STACK.lock().unwrap().get(var2) {
                                                return Expression::Operation(Operator::Multiply(*num1, *var2_val, Some(var1.clone())))
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
                                            return Expression::Operation(Operator::Divide(*num1, *num2, Some(var1.clone())))
                                        },
                                        Token::Variable(var2) => {
                                            // condition: x = 1 / y
                                            if let Some(var2_val) = STACK.lock().unwrap().get(var2) {
                                                return Expression::Operation(Operator::Divide(*num1, *var2_val, Some(var1.clone())))
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
                _ => panic!("Expected = after a variable")
            }
        },
        Token::Print => {
            match &tokens[1] {
                Token::LeftParen => {
                    match &tokens[2] {
                        Token::Number(num1) => {
                            match &tokens[3] {
                                Token::RightParen => {
                                    return Expression::PrintVal(*num1)
                                },
                                Token::Plus => {
                                    match &tokens[4] {
                                        Token::Number(num2) => {
                                            return Expression::PrintComp(
                                                Operator::Add(*num1, *num2, None)
                                            )
                                        },
                                        Token::Variable(var) => {
                                            if let Some(var_val) = STACK.lock().unwrap().get(var) {
                                                return Expression::PrintComp(
                                                    Operator::Add(*var_val, *num1, None)
                                                )
                                            } else {
                                                panic!("No value stored in variable {var}")
                                            }
                                        },
                                        _ => panic!("Expected either a number or a variable!")
                                    }
                                },
                                Token::Minus => {
                                    match &tokens[4] {
                                        Token::Number(num2) => {
                                            return Expression::PrintComp(
                                                Operator::Subtract(*num1, *num2, None)
                                            )
                                        },
                                        Token::Variable(var) => {
                                            if let Some(var_val) = STACK.lock().unwrap().get(var) {
                                                return Expression::PrintComp(
                                                    Operator::Subtract(*var_val, *num1, None)
                                                )
                                            } else {
                                                panic!("No value stored in variable {var}")
                                            }
                                        },
                                        _ => panic!("Expected either a number or a variable!")
                                    }
                                },
                                Token::Star => {
                                    match &tokens[4] {
                                        Token::Number(num2) => {
                                            return Expression::PrintComp(
                                                Operator::Multiply(*num1, *num2, None)
                                            )
                                        },
                                        Token::Variable(var) => {
                                            if let Some(var_val) = STACK.lock().unwrap().get(var) {
                                                return Expression::PrintComp(
                                                    Operator::Multiply(*var_val, *num1, None)
                                                )
                                            } else {
                                                panic!("No value stored in variable {var}")
                                            }
                                        },
                                        _ => panic!("Expected either a number or a variable!")
                                    }
                                },
                                Token::Slash => {
                                    match &tokens[4] {
                                        Token::Number(num2) => {
                                            return Expression::PrintComp(
                                                Operator::Divide(*num1, *num2, None)
                                            )
                                        },
                                        Token::Variable(var) => {
                                            if let Some(var_val) = STACK.lock().unwrap().get(var) {
                                                return Expression::PrintComp(
                                                    Operator::Divide(*var_val, *num1, None)
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
                            if let Some(var_val) = STACK.lock().unwrap().get(var) {
                                match &tokens[3] {
                                    Token::RightParen => {
                                        return Expression::PrintVal(*var_val)
                                    },
                                    Token::Plus => {
                                        match &tokens[4] {
                                            Token::Number(num1) => {
                                                match &tokens[5] {
                                                    Token::RightParen => {
                                                        return Expression::PrintComp(
                                                            Operator::Add(*var_val, *num1, None)
                                                        )
                                                    },
                                                    _ => panic!("Expected a ) at the end")
                                                }
                                            },
                                            Token::Variable(var2) => {
                                                match &tokens[6] {
                                                    Token::RightParen => {
                                                        if let Some(var2_val) = STACK.lock().unwrap().get(var2) {
                                                            return Expression::PrintComp(
                                                                Operator::Add(*var_val, *var2_val, None)
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
                                                            Operator::Subtract(*var_val, *num1, None)
                                                        )
                                                    },
                                                    _ => panic!("Expected a ) at the end")
                                                }
                                            },
                                            Token::Variable(var2) => {
                                                match &tokens[6] {
                                                    Token::RightParen => {
                                                        if let Some(var2_val) = STACK.lock().unwrap().get(var2) {
                                                            return Expression::PrintComp(
                                                                Operator::Subtract(*var_val, *var2_val, None)
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
                                                            Operator::Multiply(*var_val, *num1, None)
                                                        )
                                                    },
                                                    _ => panic!("Expected a ) at the end")
                                                }
                                            },
                                            Token::Variable(var2) => {
                                                match &tokens[6] {
                                                    Token::RightParen => {
                                                        if let Some(var2_val) = STACK.lock().unwrap().get(var2) {
                                                            return Expression::PrintComp(
                                                                Operator::Multiply(*var_val, *var2_val, None)
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
                                                            Operator::Divide(*var_val, *num1, None)
                                                        )
                                                    },
                                                    _ => panic!("Expected a ) at the end")
                                                }
                                            },
                                            Token::Variable(var2) => {
                                                match &tokens[6] {
                                                    Token::RightParen => {
                                                        if let Some(var2_val) = STACK.lock().unwrap().get(var2) {
                                                            return Expression::PrintComp(
                                                                Operator::Divide(*var_val, *var2_val, None)
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
                    match &tokens[2] {
                        Token::Eof => {
                            return Expression::PrintVal(*num)
                        },
                        _ => panic!("Got an unexpected value after {num}")
                    }
                },
                Token::Variable(var) => {
                    match &tokens[2] {
                        Token::Eof => {
                            if let Some(var_val) = STACK.lock().unwrap().get(var) {
                                return Expression::PrintVal(*var_val)
                            } else {
                                panic!("The variable {var} has no value!")
                            }
                        },
                        _ => panic!("Got an unexpected value after {var}")
                    }
                }
                _ => panic!("Expected either parantheses or a value/variable")
            }
        }
        _ => panic!("Expected either a keyword or variable")
    }
}

fn evaluate(expr: Expression) -> Option<i64> {
    fn handle_operation(operator: Operator) -> i64 {
        match operator {
            Operator::Insert(variable, num) => {
                let mut lock = STACK.lock().unwrap();
                lock.insert(variable, num);
                num
            }
            Operator::Add(num1, num2, variable) => {
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
            Operator::Subtract(num1, num2, variable) => {
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
            Operator::Multiply(num1, num2, variable) => {
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
            Operator::Divide(num1, num2, variable) => {
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
        }
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

        // println!("Line: {}", line);

        let tokens = tokenize(&line);
        // println!("Tokens: {:?}", &tokens);
        let parsed = parser(&tokens);
        // println!("Parsed: {:?}", &parsed);
        if let Some(result) = evaluate(parsed) {
            println!("Result of {} is {}", line, result)
        }
    }
}

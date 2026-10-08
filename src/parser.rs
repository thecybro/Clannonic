
use crate::{
    Parser,
    Expression,
    Operator,
    Token,
    Store,
};

impl Parser {
    pub(crate) fn new(tokens: &[Token]) -> Expression {
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
                if length_of_tokens >= 1 {
                    panic!("You can't put multiple tokens after print if there's no '('")
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
                        if length_of_tokens <= 2 {
                            panic!("Expected just a variable or number when there is no '('")
                        }
                        return Expression::PrintVal(*num)
                    },
                    Token::Variable(var) => {
                        if let Some(var_val) = store.get(var) {
                            if length_of_tokens >= 2 {
                                panic!("You can't put multiple tokens after print if there's no '('")
                            }
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

use crate::{
    Token,
    Lexer,
};

impl Lexer {
    pub(crate) fn new(input: &str) -> Self {
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
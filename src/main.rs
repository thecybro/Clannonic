
use std::{
    io::{
        self,
        Write,
        BufRead,
        BufReader,
    },
    fs::File,
};

enum Token {
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash
}

enum Expression {
    Add(i64, i64),
    Subtract(i64, i64),
    Multiply(i64, i64),
    Divide(i64, i64)
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
            _ => panic!("Unsupported token!")
        }).collect::<Vec<_>>();

    tokens
}

fn parse(tokens: &[Token]) -> Expression {
    let left = match tokens[0] {
        Token::Number(num) => num,
        _ => panic!("Expected a number!")
    };

    let right = match tokens[2] {
        Token::Number(num) => num,
        _ => panic!("Expected a number!")
    };
    
    match tokens[1] {
        Token::Plus => Expression::Add(left, right),
        Token::Minus => Expression::Subtract(left, right),
        Token::Star => Expression::Multiply(left, right),
        Token::Slash => Expression::Divide(left, right),
        _ => panic!("Expected either from + - * /"),
    }
}

fn evaluate(expr: Expression) -> i64 {
    match expr {
        Expression::Add(num1, num2) => num1 + num2,
        Expression::Subtract(num1, num2) => num1 - num2,
        Expression::Multiply(num1, num2) => num1 * num2,
        Expression::Divide(num1, num2) => num1 / num2,
    }
}

fn main() {
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

        let tokens = tokens(line.as_str());
        let expression = parse(&tokens);
        let result = evaluate(expression);
        println!("Result of {} is {}", line, result)
    }
}
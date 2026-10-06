
use std::{
    io::{
        self,
        Write,
        BufRead,
        BufReader,
    },
    fs::{File},
};

#[derive(Debug)]
enum Token {
    Number(i64),
    Plus
}

#[derive(Debug)]
struct Lexer {
    tokens: Vec<Token>
}

enum Expr {
    Add(i64, i64)
}

fn evaluate(expr: Expr) -> i64 {
    match expr {
        Expr::Add(a, b) => a + b,
    }
}

fn parse(tokens: &[Token]) -> Expr {
    let left = match tokens[0] {
        Token::Number(n) => n,
        _ => panic!("Expected number!")
    };

    match tokens[1] {
        Token::Plus => {},
        _ => panic!("Expected +"),
    }

    let right = match tokens[2] {
        Token::Number(n) => n,
        _ => panic!("Expected number")
    };

    Expr::Add(left, right)
}

impl Lexer {
    fn new(input: &str) -> Self {
        let tokens = input
            .chars()
            .filter(|t| !t.is_ascii_whitespace())
            .map(|c| match c {
                '0'..'9' => {
                    match c.to_string().parse::<i64>() {
                        Ok(num) => Token::Number(num),
                        Err(_) => panic!("Couldn't parse character as number")
                    }
                },
                '+' => Token::Plus,
                _ => panic!("Unsupported token!")
            }).collect::<Vec<_>>();
        Self{tokens}
    }
}

fn main() {
    println!("Enter path of file to analyze");
    print!(">> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read input");

    let file = File::open(&input.trim()).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let lexed = Lexer::new(line.as_str());
        let expr = parse(&lexed.tokens);
        let result = evaluate(expr);
        println!("Result of {} is {}", line, result)
    }
    
}
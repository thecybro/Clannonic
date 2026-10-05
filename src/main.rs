
use std::{
    io::{
        self,
        Write,
        BufRead,
        BufReader,
    },
    fs::File,
};

#[derive(Debug)]
enum Token {
    Number(i64), // 1
    Identifier(char), // =

    Variable(char),
    
    Plus,
    Minus,
    Star,
    Slash,

    LeftParen,
    RightParen,

    Print,
}

#[derive(Debug)]
struct Lexer {
    tokens: Vec<Token>
}

impl Lexer {
    fn is_print(input: &str) -> bool {
        let mut iter = input.chars().peekable();
        let keyword_start = 'p';
        
        while let Some(&c) = iter.peek() {
            if c == keyword_start {
                let mut word = String::new();
                while let Some(&next_c) = iter.peek() {
                    if next_c.is_alphabetic() {
                        word.push(iter.next().unwrap())
                    }
                }
                return true
            } else {
                return false
            }
        }
        panic!("Couldn't iterate over characters")
    }
    
    fn new(input: &str) -> Self {
        let tokens = input
            .chars()
            .peekable()
            .filter(|t| !t.is_ascii_whitespace())
            .map(|c| match c {
                '0'..'9' => {
                    let num: i64 = match c.to_string().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Couldn't parse the input '{}' to a number", c);
                            panic!("Panicking..")
                        }
                    };
                    Token::Number(num)
                },
                '=' => Token::Identifier(c),
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                'p' => {
                    if Self::is_print(input) {
                        Token::Print
                    } else {
                        Token::Variable(c)
                    }
                },
                'a'..'z' | 'A'..'Z' => Token::Variable(c),
                _ => panic!("Found unexpected token!")
            }).collect::<Vec<_>>();

        Self {
            tokens
        }
    }
}

fn main() {
    println!("Enter filename of the file to analyze");
    print!(">> ");

    io::stdout().flush().unwrap();
    
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read input!");

    println!("Input is: {}", input);

    let input = input.trim();

    let file = File::open(&input).expect("Couldn't open file");
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line.unwrap();
        let lexed = Lexer::new(&line);
        println!("{lexed:?}");
    }
}
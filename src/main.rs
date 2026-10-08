
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


mod parser;
mod lexer;
mod evaluater;
mod store;

static STACK: LazyLock<Mutex<HashMap<String, f64>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new()) // start with no elements
});

struct Store;

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

fn main() {
    let mut input = String::new();

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

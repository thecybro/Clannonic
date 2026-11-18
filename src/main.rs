//! The language that aims to give you simplicity like C/zig
//! while removing as much overhead as possible between you and the OS,
//! while also making it harder for you to make mistakes by making
//! you control the data flow explicitly and with intent, so you
//! dont have to fight the borrow checker which gets annoying sometimes


use std::{
    env,
    fs,
};

/// A simple glue for the lexer and parser (and evaluater/code generator later)
/// which calls and takes return value from one and gives it to other
fn main() -> rupple::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut path = String::new();

    
    if args.len() > 1 {
        path = args[1].clone();
    } else {
        println!("Rupple")
    }

    let msg = format!("Couldn't read file {path}");

    let source = fs::read_to_string(path.trim()).expect(&msg);

    rupple::run(&source)
}

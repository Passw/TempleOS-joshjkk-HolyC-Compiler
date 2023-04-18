mod lexer;
use lexer::*;

mod token;
use token::*;

use std::fs::read_to_string;
use std::env::args;

fn main() {
    let filename = args().nth(1).expect("Expected <file_to_compile>");
    let mut src = read_to_string(filename).unwrap();
    src.push('\0'); /* add a terminating char for the lexer */
    Lexer::new(src).lex();
}
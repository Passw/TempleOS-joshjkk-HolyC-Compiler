mod lexer;
use lexer::*;

mod token;
use token::*;

mod parser;
use parser::*;

mod ast;
use ast::*;

use std::fs::read_to_string;
use std::env::args;

fn main() {
    let filename = args().nth(1).expect("Expected <file_to_compile>");
    let mut src = read_to_string(filename).unwrap();
    src.push('\0'); /* add a terminating char for the lexer */

    let lex = Lexer::new(src);
    let mut parse = Parser::new(lex);
    parse.parse();
}
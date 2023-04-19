mod lexer;
use lexer::*;

mod token;
use token::*;

mod parser;
use parser::*;

mod ast;
use ast::*;

mod asm;
use asm::*;

use std::fs::{read_to_string, File};
use std::env::args;
use std::io::prelude::*;


fn main() {
    let filename = args().nth(1).expect("Expected <file_to_compile>");
    compile(&filename);
}

fn compile(filename: &String) {
    let mut src = read_to_string(filename).unwrap();
    src.push('\0'); /* add a terminating char for the lexer */

    let lex = Lexer::new(src);
    let mut parse = Parser::new(lex);
    let root = asm_f_root(&parse.parse());

    let out_filename: Vec<&str> = filename.split(".").collect();
    let mut asm_file = File::create(format!("{}.asm", out_filename.first().unwrap())).expect("Error creating to output file");
    
    asm_file.write_all(root.as_bytes()).expect("Error writing to out file");
}
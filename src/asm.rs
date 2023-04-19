use crate::{AST, ASTType};

fn asm_f_compound(ast: &AST) -> String {
    let mut value = String::new();

    for child in ast.children.as_ref() {
        let next_value = asm_f(&child);
        value.push_str(&next_value);
    }

    return value;
}

/* TODO:
fn asm_f_var(ast: &AST) -> String {
}
*/

/*
fn asm_f_function(ast: &AST) -> String {
}
*/

fn asm_f_call(ast: &AST) -> String {
    let asm = format!("global {}\n{}:\n", ast.name, ast.name);
    // TODO: do stuff in the function
    return asm;
}

/* TODO:
fn asm_f_int(ast: &AST) -> String {
}
*/

fn asm_f(ast: &AST) -> String {
    match ast.typ {
        ASTType::Compound => return asm_f_compound(ast),
        // TODO: ASTType::Function => return asm_f_function(ast),
        // TODO: ASTType::Var => return asm_f_var(ast),
        ASTType::Call => return asm_f_call(ast),
        // TODO: ASTType::Int => return asm_f_int(ast),
        _ => panic!("{}", &format!("[ASM]: No frontend for AST of type '{:?}'", ast.typ))
    }
}

pub fn asm_f_root(ast: &AST) -> String {
    let section_text = "bits 64\nsection .text\nglobal _start\n_start:\ncall main\nmov rax, 60\nmov rdi, 0\nsyscall\nret\n\n";
    
    let mut value = section_text.clone().to_string();
    let next_value = asm_f(ast);
    value.push_str(&next_value);

    return value;
}
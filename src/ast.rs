#[derive(Debug)]
#[derive(PartialEq)]
pub enum ASTType {
    Compound,
    Function,
    Call,
    //DefType,
    Assignment,
    Var,
    Int,
    //Statement
}

#[derive(Debug)]
pub struct AST {
    pub typ: ASTType,
    pub children: Box<Vec<AST>>,
    pub name: String,
    pub value: Box<Vec<AST>>,
    pub int_value: i64
}

impl AST {
    pub fn new(ast_type: ASTType) -> Self {
        Self {
            typ: ast_type,
            children: Box::new(Vec::new()),
            name: String::new(),
            value: Box::new(Vec::new()),
            int_value: 0
        }
    }
}
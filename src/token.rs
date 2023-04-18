pub enum TokenType {
    ID,
    LParen,
    RParen,
    Comma,
    Star,
    LSquare,
    RSquare,
    LBrace,
    RBrace,
    Equals,
    Int,
    Semi,
    EOF
}

impl TokenType {
    pub fn to_string(&self) -> &str {
        match self {
            TokenType::ID => "ID",
            TokenType::LParen => "LParen",
            TokenType::RParen => "RParen",
            TokenType::Comma => "Comma",
            TokenType::Star => "Star",
            TokenType::LSquare => "LSquare",
            TokenType::RSquare => "RSquare",
            TokenType::LBrace => "LBrace",
            TokenType::RBrace => "RBrace",
            TokenType::Equals => "Equals",
            TokenType::Int => "Int",
            TokenType::Semi => "Semi",
            TokenType::EOF => "EOF"
        }
    }
}

pub struct Token {
    pub value: String,
    pub tok_type: TokenType
}

impl Token {
    pub fn new(val: &str, typ: TokenType) -> Self {
        Self {
            value: val.to_string(),
            tok_type: typ
        }
    }

    pub fn to_string(&self) -> String {
        format!("<Token type='{}', value='{}'>", self.tok_type.to_string(), self.value)
    }
}
use crate::{Token, TokenType};

pub struct Lexer {
    src: String,
    i: usize,
    c: char
}

impl Lexer {
    pub fn new(file_src: String) -> Self {
        Self {
            src: file_src.clone(),
            i: 0,
            c: file_src.chars().nth(0).unwrap()
        }
    }

    fn advance(&mut self) {
        if self.i < self.src.len() && self.c != '\0' {
            self.i += 1;
            self.c = self.src.chars().nth(self.i).unwrap();
        }
    }

    fn advance_cur(&mut self, val: &str, typ: TokenType) -> Token {
        self.advance();
        Token::new(val, typ)
    }

    fn skip_whitespace(&mut self) {
        while self.c == ' ' || self.c == '\r' || self.c == '\t' || self.c == '\n' {
            self.advance();
        }
    }

    fn peek(&self, offset: i64) -> char {
        return self.src.chars().nth((self.i as i64 + offset) as usize).expect("Index out of range")
    }

    fn lex_id(&mut self) -> Token {
        let mut value = String::new();
        while self.c.is_alphabetic() || self.c.is_alphanumeric() || self.c == '_' {
            value.push(self.c);
            self.advance();
        }
        Token::new(&value, TokenType::ID)
    }

    fn lex_int(&mut self) -> Token {
        let mut value = String::new();
        while self.c.is_ascii_alphanumeric() {
            value.push(self.c);
            self.advance();
        }
        Token::new(&value, TokenType::Int)
    }

    pub fn next_tok(&mut self) -> Token {
        while self.c != '\0' { /* last char is '\0' */
            self.skip_whitespace();

            if self.c.is_alphabetic() || (self.c.is_alphabetic() && self.peek(1).is_alphanumeric()) {
                return self.lex_id();
            } else if self.c.is_alphanumeric() {
                return self.lex_int();
            }

            match self.c {
                '(' => return self.advance_cur("(", TokenType::LParen),
                ')' => return self.advance_cur(")", TokenType::RParen),
                ',' => return self.advance_cur(",", TokenType::Comma),
                '*' => return self.advance_cur("*", TokenType::Star),
                '[' => return self.advance_cur("[", TokenType::LSquare),
                ']' => return self.advance_cur("]", TokenType::RSquare),
                '{' => return self.advance_cur("{", TokenType::LBrace),
                '}' => return self.advance_cur("}", TokenType::RBrace),
                '=' => return self.advance_cur("=", TokenType::Equals),
                ';' => return self.advance_cur(";", TokenType::Semi),
                _ => panic!("[Lexer]: Unexpected token '{}'", self.c)
            }
        }
        Token::new("NOT_STRINGABLE", TokenType::EOF)
    }
}
use crate::{Lexer, Token, TokenType, AST, ASTType};

pub struct Parser {
    lex: Lexer,
    tok: Token
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let token = lexer.next_tok();
        Self {
            lex: lexer,
            tok: token
        }
    }

    fn eat(&mut self, typ: TokenType) {
        if self.tok.tok_type != typ {
            panic!("{}", &format!("[Parser]: Unexpected token type '{}', was expecting type '{}'", 
                self.tok.tok_type.to_string(), typ.to_string()));
        }

        println!("{}", self.tok.to_string());
        self.tok = self.lex.next_tok();
    }

    fn parse_id(&mut self) -> AST {
        let value = String::from(self.tok.value.as_mut_str());
        let mut next_value = String::new();

        while self.tok.tok_type == TokenType::ID {
            self.eat(TokenType::ID);

            while self.tok.tok_type == TokenType::Star {
                self.eat(TokenType::Star);
            }

            if self.tok.tok_type == TokenType::LSquare {
                self.eat(TokenType::LSquare);
                self.eat(TokenType::RSquare);
            }

            if self.tok.tok_type == TokenType::ID {
                next_value = String::from(self.tok.value.as_mut_str());
                self.eat(TokenType::ID);

                if self.tok.tok_type == TokenType::Equals {
                    self.eat(TokenType::Equals);

                    let mut ast = AST::new(ASTType::Assignment);
                    ast.name = next_value;

                    ast.value = Box::new(Vec::new());
                    ast.value.push(self.parse_expr());

                    return ast;
                }
            }

            /* DoSomething; // no parens for no parameters */
            if self.tok.tok_type == TokenType::Semi {
                self.eat(TokenType::Semi);

                let mut ast = AST::new(ASTType::Call);
                ast.name = value;

                return ast;
            }
            
            if self.tok.tok_type == TokenType::LParen {
                let mut ast = AST::new(ASTType::Call);
                ast.name = next_value;

                ast.value = Box::new(Vec::new());
                ast.value.push(self.parse_expr());

                return ast;
            }

            let mut ast = AST::new(ASTType::Var);
            ast.name = value;

            ast.value = Box::new(Vec::new());
            ast.value.push(self.parse_expr());

            return ast;
        }

        panic!("{}", &format!("[Parser]: Error parsing token '{}'", self.tok.to_string()));
    }

    fn parse_int(&mut self) -> AST {
        let mut ast = AST::new(ASTType::Int);
        ast.int_value = self.tok.value.parse::<i64>().unwrap();

        self.eat(TokenType::Int);

        return ast;
    }

    fn parse_block(&mut self) -> AST {
        self.eat(TokenType::LBrace);
        let mut ast = AST::new(ASTType::Compound);

        while self.tok.tok_type != TokenType::RBrace {
            ast.children.push(self.parse_expr());
            if self.tok.tok_type == TokenType::Semi {
                self.eat(TokenType::Semi);
            }
        }

        self.eat(TokenType::RBrace);

        return ast;
    }

    fn parse_list(&mut self) -> AST {
        self.eat(TokenType::LParen);
        let mut ast = AST::new(ASTType::Compound);

        if self.tok.tok_type != TokenType::RParen {
            ast.children.push(self.parse_expr());
            while self.tok.tok_type == TokenType::Comma {
                self.eat(TokenType::Comma);
                ast.children.push(self.parse_expr());
            }
        }

        self.eat(TokenType::RParen);

        if self.tok.tok_type == TokenType::LBrace {
            ast.typ = ASTType::Function;
        }

        ast.value = Box::new(Vec::new());
        ast.value.push(self.parse_block());

        return ast;
    }

    fn parse_expr(&mut self) -> AST {
        match self.tok.tok_type {
            TokenType::ID => self.parse_id(),
            TokenType::Int => self.parse_int(),
            TokenType::LParen => self.parse_list(),
            _ => panic!("{}", &format!("[Parser]: Unexpected token '{}'\n", self.tok.to_string()))
        }
    }

    fn parse_compound(&mut self) -> AST {
        let mut ast = AST::new(ASTType::Compound);

        while self.tok.tok_type != TokenType::EOF {
            ast.children.push(self.parse_expr());

            if self.tok.tok_type == TokenType::Semi {
                self.eat(TokenType::Semi);
            }
        }

        return ast;
    }

    pub fn parse(&mut self) -> AST {
        self.parse_compound() /* main */
    }
}
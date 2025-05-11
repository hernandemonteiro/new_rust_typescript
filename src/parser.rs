use crate::ast::{BinOp, Expr};
use crate::lexer::Token;

pub struct Parser<'a> {
    lexer: logos::Lexer<'a, Token>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: logos::Lexer<'a, Token>) -> Self {
        let current_token = lexer.next().and_then(Result::ok);
        Parser {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next().and_then(Result::ok);
    }

    pub fn parse(&mut self) -> Option<Vec<Expr>> {
        let mut exprs = Vec::new();

        while let Some(expr) = self.parse_stmt() {
            exprs.push(expr);
        }

        if exprs.is_empty() { None } else { Some(exprs) }
    }

    fn parse_stmt(&mut self) -> Option<Expr> {
        match &self.current_token {
            Some(Token::Let) => self.parse_let(),
            Some(Token::Print) => self.parse_print(),
            Some(Token::Identifier) => self.call_expr(),
            Some(Token::StringLiteral) => {
                let value = self.lexer.slice().to_string();
                self.advance();
                value.to_string();
                Some(Expr::String(value))
            }
            _ => self.expr(),
        }
    }

    fn parse_let(&mut self) -> Option<Expr> {
        self.advance();

        let name = if let Some(Token::Identifier) = &self.current_token {
            let name = self.lexer.slice().to_string();
            self.advance();
            name
        } else {
            return None;
        };

        if self.current_token != Some(Token::Equals) {
            return None;
        }
        self.advance();

        let value = self.expr()?;

        if self.current_token != Some(Token::Semicolon) {
            return None;
        }
        self.advance();

        Some(Expr::Assign(name, Box::new(value)))
    }

    fn parse_print(&mut self) -> Option<Expr> {
        self.advance();

        if self.current_token != Some(Token::LParen) {
            return None;
        }
        self.advance();

        let expr = self.expr()?;

        if self.current_token != Some(Token::RParen) {
            return None;
        }
        self.advance();

        if self.current_token != Some(Token::Semicolon) {
            return None;
        }
        self.advance();

        Some(Expr::Print(Box::new(expr)))
    }

    fn call_expr(&mut self) -> Option<Expr> {
        if let Some(Token::Identifier) = self.current_token {
            let func_name = self.lexer.slice().to_string();
            self.advance();

            if let Some(Token::LParen) = self.current_token {
                self.advance();

                let mut args = Vec::new();

                while let Some(arg) = self.term() {
                    args.push(arg);

                    if let Some(Token::Comma) = self.current_token {
                        self.advance();
                    } else {
                        break;
                    }
                }

                if let Some(Token::RParen) = self.current_token {
                    self.advance();
                    return Some(Expr::Call(func_name, args));
                }
            }
        }

        None
    }

    fn expr(&mut self) -> Option<Expr> {
        let mut left = self.term()?;

        while let Some(token) = &self.current_token {
            match token {
                Token::Plus => {
                    self.advance();
                    let right = self.term()?;
                    left = Expr::BinOp(Box::new(left), BinOp::Add, Box::new(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.term()?;
                    left = Expr::BinOp(Box::new(left), BinOp::Sub, Box::new(right));
                }
                Token::Star => {
                    self.advance();
                    let right = self.term()?;
                    left = Expr::BinOp(Box::new(left), BinOp::Mul, Box::new(right));
                }
                Token::Slash => {
                    self.advance();
                    let right = self.term()?;
                    left = Expr::BinOp(Box::new(left), BinOp::Div, Box::new(right));
                }
                _ => break,
            }
        }
        Some(left)
    }

    fn term(&mut self) -> Option<Expr> {
        let token = self.current_token.clone()?;

        match token {
            Token::Number => {
                let value = self.lexer.slice().parse::<i64>().ok()?;
                self.advance();
                Some(Expr::Number(value))
            }
            Token::Identifier => {
                let name = self.lexer.slice().to_string();
                self.advance();
                Some(Expr::Var(name))
            }
            Token::StringLiteral => {
                let value = self.lexer.slice().trim_matches('"').to_string();
                self.advance();
                Some(Expr::String(value))
            }
            _ => None,
        }
    }
}

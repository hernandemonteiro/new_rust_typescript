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

    pub fn parse(&mut self) -> Option<Expr> {
        self.parse_stmt()
    }

    fn parse_stmt(&mut self) -> Option<Expr> {
        match &self.current_token {
            Some(Token::Let) => self.parse_let(),
            Some(Token::Print) => self.parse_print(),
            Some(Token::Identifier) => self.call_expr(),
            _ => self.expr(),
        }
    }

    fn parse_let(&mut self) -> Option<Expr> {
        self.advance(); // consumir 'let'

        let name = if let Some(Token::Identifier) = &self.current_token {
            let name = self.lexer.slice().to_string(); // pega nome real
            self.advance();
            name
        } else {
            return None;
        };

        if self.current_token != Some(Token::Equals) {
            return None;
        }
        self.advance(); // consumir '='

        let value = self.expr()?; // parse do valor da atribuição

        if self.current_token != Some(Token::Semicolon) {
            return None;
        }
        self.advance(); // consumir ';'

        Some(Expr::Assign(name, Box::new(value)))
    }

    fn parse_print(&mut self) -> Option<Expr> {
        self.advance(); // consumir 'print'

        if self.current_token != Some(Token::LParen) {
            return None;
        }
        self.advance(); // consumir '('

        let expr = self.expr()?;

        if self.current_token != Some(Token::RParen) {
            return None;
        }
        self.advance(); // consumir ')'

        if self.current_token != Some(Token::Semicolon) {
            return None;
        }
        self.advance(); // consumir ';'

        Some(Expr::Print(Box::new(expr)))
    }

    fn call_expr(&mut self) -> Option<Expr> {
        // Espera que o token atual seja um identificador (nome da função)
        if let Some(Token::Identifier) = self.current_token {
            let func_name = self.lexer.slice().to_string(); // Pega o nome da função
            self.advance(); // Avança para o próximo token, que deve ser '('

            // Agora, precisamos verificar os parênteses e os argumentos
            if let Some(Token::LParen) = self.current_token {
                self.advance(); // Avança para o próximo token (deve ser o primeiro argumento ou ')')

                let mut args = Vec::new();

                // Parse de argumentos
                while let Some(arg) = self.term() {
                    args.push(arg);

                    // Se o próximo token for uma vírgula, avança para o próximo argumento
                    if let Some(Token::Comma) = self.current_token {
                        self.advance();
                    } else {
                        break;
                    }
                }

                // Verifica se temos o parêntese de fechamento
                if let Some(Token::RParen) = self.current_token {
                    self.advance(); // Avança para o próximo token
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
            _ => None,
        }
    }
}

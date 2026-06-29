use crate::lexer::Lexer;
use crate::token::Token;
use crate::{ast, token::Token::LParen};
use ast::{Expr, Op};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    pub fn eat(&mut self, expected: Token) {
        if self.current_token == expected {
            self.current_token = self.lexer.next_token();
        } else {
            panic!("Unexpected token: {:?}", self.current_token);
        }
    }

    // 解析表达式（处理加减）
    pub fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_term();
        loop {
            match self.current_token {
                Token::Plus => {
                    self.eat(Token::Plus);
                    let right = self.parse_term();
                    left = Expr::Binary {
                        op: Op::Add,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.eat(Token::Minus);
                    let right = self.parse_term();
                    left = Expr::Binary {
                        op: Op::Sub,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }

                _ => break,
            }
        }
        left
    }

    // 解析项（处理乘除）
    pub fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();
        loop {
            match self.current_token {
                Token::Star => {
                    self.eat(Token::Star);
                    let right = self.parse_factor();
                    left = Expr::Binary {
                        op: Op::Mul,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.eat(Token::Slash);
                    let right = self.parse_factor();
                    left = Expr::Binary {
                        op: Op::Div,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        left
    }

    // 解析因子（数字或括号表达式）
    pub fn parse_factor(&mut self) -> Expr {
        match self.current_token {
            Token::Number(n) => {
                self.eat(Token::Number(n));
                Expr::Number(n)
            }

            Token::LParen => {
                self.eat(LParen);
                let expr = self.parse_expr();
                self.eat(Token::RParen);
                expr
            }

            _ => panic!("Unexpected token in factor: {:?}", self.current_token),
        }
    }
}

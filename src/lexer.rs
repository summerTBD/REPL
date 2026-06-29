use crate::token::Token;

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            chars: input.chars().collect::<Vec<char>>(),
            pos: 0,
        }
    }

    // 查看当前的字符
    pub fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.peek();
        self.pos += 1;
        // return value
        c
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.next();
        }
    }

    pub fn read_number(&mut self) -> f64 {
        let mut num_str = String::new();
        while let Some(c) = self.peek() {
            if c.is_digit(10) || c == '.' {
                num_str.push(c);
                self.next();
            } else {
                break;
            }
        }

        num_str.parse::<f64>().unwrap_or(0.0)
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.peek() {
            None => Token::EOF,
            Some(c) => match c {
                '+' => {
                    self.next();
                    Token::Plus
                }
                '-' => {
                    self.next();
                    Token::Minus
                }
                '*' => {
                    self.next();
                    Token::Star
                }
                '/' => {
                    self.next();
                    Token::Slash
                }
                '(' => {
                    self.next();
                    Token::LParen
                }
                ')' => {
                    self.next();
                    Token::RParen
                }
                _ if c.is_digit(10) || c == '.' => {
                    let num = self.read_number();
                    Token::Number(num)
                }

                _ => {
                    self.next();
                    Token::EOF
                }
            },
        }
    }
}

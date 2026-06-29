mod AST;
mod lexer;
mod parser;
mod token;

use crate::{
    AST::{Expr, Op},
    lexer::Lexer,
    parser::Parser,
};

use std::io::{self, Write};

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,

        Expr::Binary { op, left, right } => {
            let left_val = eval(left);
            let right_val = eval(right);
            match op {
                Op::Add => left_val + right_val,
                Op::Sub => left_val - right_val,
                Op::Mul => left_val * right_val,
                Op::Div => {
                    if right_val == 0.0 {
                        panic!("Division by zero");
                    }
                    left_val / right_val
                }
            }
        }
    }
}

fn main() {
    println!("Rust Calculator (type 'quit' to exit)");
    loop {
        print!(">");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "quite" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        let expr = parser.parse_expr();

        match eval(&expr) {
            result => println!("= {}", result),
        }
    }
}

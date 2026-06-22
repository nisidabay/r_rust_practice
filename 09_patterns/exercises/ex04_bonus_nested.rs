// ex04_bonus_nested.rs — Parse nested expressions "(add 1 (mul 2 3))"
// Simple recursive descent with match. Supports: add, sub, mul, div.

use std::io::{self, BufRead};

// A simple recursive expression parser
// Grammar:
//   expr  = "(" op arg arg ")"  | number
//   op    = "add" | "sub" | "mul" | "div"
//   arg   = expr | number

#[derive(Debug, Clone)]
enum Expr {
    Num(f64),
    BinOp {
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    }
}

// Tokenize input string into tokens: (, ), add, sub, mul, div, numbers
fn tokenize(s: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for c in s.chars() {
        if c == '(' || c == ')' {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
            tokens.push(c.to_string());
        } else if c.is_whitespace() {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

// Recursive descent parser
struct Parser {
    tokens: Vec<String>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<String>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.pos).map(|s| s.as_str())
    }

    fn consume(&mut self) -> Option<String> {
        let t = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        t
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        match self.peek()? {
            "(" => {
                self.consume(); // consume '('
                let op = self.consume()?; // consume operator
                let left = self.parse_expr()?;
                let right = self.parse_expr()?;
                // expect closing paren
                if self.peek() != Some(")") {
                    return None;
                }
                self.consume(); // consume ')'
                Some(Expr::BinOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            n if n.chars().all(|c| c.is_digit(10) || c == '.') => {
                let num = self.consume()?;
                Some(Expr::Num(num.parse().ok()?))
            }
            _ => None,
        }
    }
}

fn eval(expr: &Expr) -> Option<f64> {
    match expr {
        Expr::Num(n) => Some(*n),
        Expr::BinOp { op, left, right } => {
            let l = eval(left)?;
            let r = eval(right)?;
            match op.as_str() {
                "add" => Some(l + r),
                "sub" => Some(l - r),
                "mul" => Some(l * r),
                "div" => {
                    if r == 0.0 { None } else { Some(l / r) }
                }
                _ => None,
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    println!("Enter expression like: (add 1 (mul 2 3))");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }
        let tokens = tokenize(&line);
        let mut parser = Parser::new(tokens);
        if let Some(expr) = parser.parse_expr() {
            match eval(&expr) {
                Some(result) => println!("= {}", result),
                None => println!("Error: division by zero"),
            }
        } else {
            println!("Error: invalid expression");
        }
    }
}

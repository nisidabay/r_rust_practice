// json_tokenizer — A CLI that reads JSON from stdin and tokenizes it.
//
// Demonstrates ownership transfer through parse stages:
//   1. Read stdin → owned String
//   2. Tokenize → returns Vec<Token> (ownership of tokens, but String is consumed)
//   3. Display tokens → borrows Vec<Token>
//
// Usage:
//   echo '{"hello": 42}' | cargo run
//   cargo run -- --help

use std::env;
use std::io::{self, Read};

// ─── Token types ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
enum Token {
    BraceOpen,   // {
    BraceClose,  // }
    BracketOpen, // [
    BracketClose, // ]
    Colon,       // :
    Comma,       // ,
    String(String), // "..." — owns its content
    Number(f64), // numeric literal
    True,
    False,
    Null,
}

// ─── Tokenizer ───────────────────────────────────────────────────────────────

// Tokenizer PRODUCES tokens from an owned String.
// The String is moved in (consumed), and ownership of parsed String tokens
// is transferred to the Vec<Token>.
struct Tokenizer {
    // The JSON input — owned by the Tokenizer during its lifetime
    input: String,
    // Current position in the input
    pos: usize,
}

impl Tokenizer {
    fn new(input: String) -> Self {
        Tokenizer { input, pos: 0 }
    }

    // Tokenize the entire input — consumes self, returns owned tokens.
    // Ownership transfer: Tokenizer owns `input` → tokens own the String parts.
    fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.pos < self.input.len() {
            self.skip_whitespace();
            if self.pos >= self.input.len() {
                break;
            }

            let ch: char = self.peek()?;
            match ch {
                '{' => {
                    tokens.push(Token::BraceOpen);
                    self.pos += 1;
                }
                '}' => {
                    tokens.push(Token::BraceClose);
                    self.pos += 1;
                }
                '[' => {
                    tokens.push(Token::BracketOpen);
                    self.pos += 1;
                }
                ']' => {
                    tokens.push(Token::BracketClose);
                    self.pos += 1;
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.pos += 1;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.pos += 1;
                }
                '"' => {
                    // String parsing — transfers ownership of parsed content
                    let s: String = self.parse_string()?;
                    tokens.push(Token::String(s));
                }
                't' => {
                    self.expect_literal("true")?;
                    tokens.push(Token::True);
                }
                'f' => {
                    self.expect_literal("false")?;
                    tokens.push(Token::False);
                }
                'n' => {
                    self.expect_literal("null")?;
                    tokens.push(Token::Null);
                }
                '-' | '0'..='9' => {
                    let num: f64 = self.parse_number()?;
                    tokens.push(Token::Number(num));
                }
                _ => {
                    return Err(format!(
                        "Unexpected character '{}' at position {}",
                        ch, self.pos
                    ));
                }
            }
        }

        Ok(tokens)
    }

    // ─── Helpers ─────────────────────────────────────────────────────────

    fn peek(&self) -> Result<char, String> {
        self.input[self.pos..].chars().next().ok_or_else(|| {
            format!("Unexpected end of input at position {}", self.pos)
        })
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let ch: char = self.input[self.pos..].chars().next().unwrap();
            if ch.is_ascii_whitespace() {
                self.pos += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    fn parse_string(&mut self) -> Result<String, String> {
        // We already consumed the opening '"', so skip it
        // Actually we haven't — peek returns '"', advance past it
        self.pos += 1; // skip opening '"'
        let start: usize = self.pos;

        while self.pos < self.input.len() {
            let ch: char = self.input[self.pos..].chars().next().unwrap();
            if ch == '"' {
                // Found closing quote — extract slice and own it
                let content: String = self.input[start..self.pos].to_string();
                self.pos += 1; // skip closing '"'
                return Ok(content);
            }
            if ch == '\\' {
                // Skip escaped character (simplified — just skip one more char)
                self.pos += ch.len_utf8();
                if self.pos < self.input.len() {
                    self.pos += 1;
                }
            } else {
                self.pos += ch.len_utf8();
            }
        }

        Err("Unterminated string literal".to_string())
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        let start: usize = self.pos;

        // Consume optional '-'
        if self.pos < self.input.len() && self.input.as_bytes()[self.pos] == b'-' {
            self.pos += 1;
        }

        // Consume digits
        while self.pos < self.input.len() && self.input.as_bytes()[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        // Consume optional fractional part
        if self.pos < self.input.len() && self.input.as_bytes()[self.pos] == b'.' {
            self.pos += 1;
            while self.pos < self.input.len() && self.input.as_bytes()[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }

        // Consume optional exponent
        if self.pos < self.input.len()
            && (self.input.as_bytes()[self.pos] == b'e' || self.input.as_bytes()[self.pos] == b'E')
        {
            self.pos += 1;
            if self.pos < self.input.len()
                && (self.input.as_bytes()[self.pos] == b'+'
                    || self.input.as_bytes()[self.pos] == b'-')
            {
                self.pos += 1;
            }
            while self.pos < self.input.len() && self.input.as_bytes()[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }

        let num_str: &str = &self.input[start..self.pos];
        num_str
            .parse::<f64>()
            .map_err(|e| format!("Failed to parse number '{}': {}", num_str, e))
    }

    fn expect_literal(&mut self, literal: &str) -> Result<(), String> {
        if self.pos + literal.len() > self.input.len() {
            return Err(format!("Expected '{}' at position {}", literal, self.pos));
        }
        let slice: &str = &self.input[self.pos..self.pos + literal.len()];
        if slice != literal {
            return Err(format!(
                "Expected '{}' at position {}, found '{}'",
                literal, self.pos, slice
            ));
        }
        self.pos += literal.len();
        Ok(())
    }
}

// ─── Display ─────────────────────────────────────────────────────────────────

// Format a token for display (borrows the token — no ownership transfer)
fn format_tokens(tokens: &[Token]) -> String {
    let mut output: String = String::new();
    output.push('[');

    for (i, token) in tokens.iter().enumerate() {
        if i > 0 {
            output.push_str(", ");
        }
        match token {
            Token::BraceOpen => output.push_str("{"),
            Token::BraceClose => output.push_str("}"),
            Token::BracketOpen => output.push_str("["),
            Token::BracketClose => output.push_str("]"),
            Token::Colon => output.push_str(":"),
            Token::Comma => output.push_str(","),
            Token::String(s) => {
                output.push('"');
                output.push_str(s);
                output.push('"');
            }
            Token::Number(n) => output.push_str(&n.to_string()),
            Token::True => output.push_str("true"),
            Token::False => output.push_str("false"),
            Token::Null => output.push_str("null"),
        }
    }

    output.push(']');
    output
}

// ─── CLI ─────────────────────────────────────────────────────────────────────

fn print_help() {
    println!("json_tokenizer 0.1.0");
    println!("A JSON tokenizer demonstrating Rust ownership transfer through parse stages");
    println!();
    println!("USAGE:");
    println!("  echo '<json>' | cargo run");
    println!("  echo '<json>' | cargo run -- --help");
    println!();
    println!("OPTIONS:");
    println!("  --help    Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("  echo '{{\"hello\": \"world\"}}' | cargo run");
    println!("  echo '[1, 2, 3]' | cargo run");
    println!();
    println!("HOW IT DEMONSTRATES OWNERSHIP:");
    println!("  1. Stdin → owned String (main owns it)");
    println!("  2. String → Tokenizer (ownership moves into Tokenizer)");
    println!("  3. Tokenizer.tokenize() → Vec<Token> (tokens own heap data)");
    println!("  4. format_tokens(&[Token]) → borrows tokens, no new ownership");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--help" {
        print_help();
        return;
    }

    // Step 1: Read stdin into an owned String
    let mut input: String = String::new();
    match io::stdin().read_to_string(&mut input) {
        Ok(0) => {
            eprintln!("Error: no input provided. Pipe JSON to stdin.");
            eprintln!("Usage: echo '<json>' | cargo run");
            std::process::exit(1);
        }
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error reading stdin: {}", e);
            std::process::exit(1);
        }
    }

    // Step 2: Ownership of `input` moves into Tokenizer
    let tokenizer: Tokenizer = Tokenizer::new(input);
    // `input` is now moved — cannot be used anymore

    // Step 3: tokenize() consumes the Tokenizer and produces owned tokens
    let tokens: Vec<Token> = match tokenizer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Tokenization error: {}", e);
            std::process::exit(1);
        }
    };
    // `tokenizer` is now consumed — cannot be used

    // Step 4: Display tokens by borrowing — no ownership transfer
    let display: String = format_tokens(&tokens);
    println!("{}", display);

    // Step 5: tokens is still owned by main, dropped at end of scope
    // Total tokens count (borrowing again)
    println!("Total tokens: {}", tokens.len());
}

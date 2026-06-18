// ex02_parser_solved.rs — Parse a string into tokens, return references to the original.
//
// Solved version of ex02_parser.

/// A token that references a slice of the parsed input.
#[derive(Debug, PartialEq)]
enum Token<'a> {
    Word(&'a str),
    Number(&'a str),
    Punctuation(char),
}

/// A simple tokenizer that borrows its input string.
struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Tokenizer { input, pos: 0 }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len()
            && self.input.as_bytes()[self.pos].is_ascii_whitespace()
        {
            self.pos += 1;
        }
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return None;
        }

        let ch = self.input.as_bytes()[self.pos];
        if ch.is_ascii_alphabetic() {
            // Consume a word
            let start = self.pos;
            while self.pos < self.input.len()
                && self.input.as_bytes()[self.pos].is_ascii_alphabetic()
            {
                self.pos += 1;
            }
            Some(Token::Word(&self.input[start..self.pos]))
        } else if ch.is_ascii_digit() {
            // Consume a number
            let start = self.pos;
            while self.pos < self.input.len()
                && self.input.as_bytes()[self.pos].is_ascii_digit()
            {
                self.pos += 1;
            }
            Some(Token::Number(&self.input[start..self.pos]))
        } else {
            // Single-character punctuation
            let c = self.input[self.pos..].chars().next().unwrap();
            self.pos += c.len_utf8();
            Some(Token::Punctuation(c))
        }
    }
}

fn main() {
    let input = "hello 42 world!";
    let mut tokenizer = Tokenizer::new(input);

    let tokens: Vec<Token> = std::iter::from_fn(|| tokenizer.next_token()).collect();

    println!("Tokens from '{input}':");
    for token in &tokens {
        println!("  {token:?}");
    }

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::Word("hello"));
    assert_eq!(tokens[1], Token::Number("42"));
    assert_eq!(tokens[2], Token::Word("world"));
    assert_eq!(tokens[3], Token::Punctuation('!'));

    // Test 2: empty input
    let empty = "";
    let mut t2 = Tokenizer::new(empty);
    assert!(t2.next_token().is_none());

    // Test 3: all whitespace
    let ws = "   \t  \n  ";
    let mut t3 = Tokenizer::new(ws);
    assert!(t3.next_token().is_none());

    // Test 4: multiple punctuation
    let punct = "?!.,";
    let mut t4 = Tokenizer::new(punct);
    let expected = ['?', '!', '.', ','];
    for &ch in &expected {
        assert_eq!(t4.next_token(), Some(Token::Punctuation(ch)));
    }
    assert!(t4.next_token().is_none());

    println!("\n✓ ex02_parser_solved passed!");
}

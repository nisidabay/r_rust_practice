// ex02_parser.rs — Parse a string into tokens, return references to the original.
//
// TODO: Implement a simple parser that borrows its input and returns tokens
// that reference the original string.
//
// This exercise demonstrates struct lifetimes: the Parser struct borrows the input,
// and the Token enum holds references to the parsed input.

/// A token that references a slice of the parsed input.
#[derive(Debug, PartialEq)]
enum Token<'a> {
    /// A word (sequence of non-whitespace characters)
    Word(&'a str),
    /// A number (sequence of digits)
    Number(&'a str),
    /// A punctuation character
    Punctuation(char),
}

/// A simple tokenizer that borrows its input string.
struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    /// Create a new tokenizer from input.
    fn new(input: &'a str) -> Self {
        Tokenizer { input, pos: 0 }
    }

    /// Skip whitespace characters.
    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len()
            && self.input.as_bytes()[self.pos].is_ascii_whitespace()
        {
            self.pos += 1;
        }
    }

    /// Check if a byte is an ASCII digit.
    fn is_digit(b: u8) -> bool {
        b.is_ascii_digit()
    }

    /// Returns the next token, or None if we've consumed all input.
    ///
    /// TODO: Implement this method.
    /// 1. Skip whitespace
    /// 2. If at end, return None
    /// 3. Peek the current character:
    ///    - If it's alphabetic, consume a Word token
    ///    - If it's a digit, consume a Number token
    ///    - Otherwise, treat it as Punctuation
    ///
    /// The returned Token references the original input string (lifetime 'a).
    fn next_token(&mut self) -> Option<Token<'a>> {
        todo!("implement token parsing")
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

    println!("\n✓ ex02_parser passed!");
}

// json_query — Simple JSON path query CLI
// WHY: Parsing JSON and extracting values is a great real-world match for
//      pattern matching on tokens, values, and paths.
//
// Usage:
//   json_query '{"a":{"b":42}}' a.b
//   => 42
//
//   json_query '{"items":[10,20,30]}' items.1
//   => 20
//
//   json_query --help

use std::process;

/// A token from the JSON lexer.
#[derive(Debug, Clone, PartialEq)]
enum Token {
    OpenBrace,     // {
    CloseBrace,    // }
    OpenBracket,   // [
    CloseBracket,  // ]
    Colon,         // :
    Comma,         // ,
    String(String), // "hello" — unquoted value inside
    Number(f64),   // 42 or 3.14
    True,          // true
    False,         // false
    Null,          // null
}

/// A parsed JSON value.
#[derive(Debug, Clone, PartialEq)]
enum JsonValue {
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

/// Lex a JSON string into tokens.
fn lex(json: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = json.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            // Whitespace: skip
            c if c.is_ascii_whitespace() => i += 1,

            // Structural characters: direct token mapping
            '{' => { tokens.push(Token::OpenBrace); i += 1; }
            '}' => { tokens.push(Token::CloseBrace); i += 1; }
            '[' => { tokens.push(Token::OpenBracket); i += 1; }
            ']' => { tokens.push(Token::CloseBracket); i += 1; }
            ':' => { tokens.push(Token::Colon); i += 1; }
            ',' => { tokens.push(Token::Comma); i += 1; }

            // String literal: "..." — parse until closing quote
            '"' => {
                i += 1; // skip opening quote
                let start = i;
                while i < chars.len() && chars[i] != '"' {
                    if chars[i] == '\\' { i += 1; } // skip escaped char
                    i += 1;
                }
                if i >= chars.len() {
                    return Err("unterminated string".into());
                }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::String(s));
                i += 1; // skip closing quote
            }

            // Number or keyword: parse the whole word
            c if c.is_ascii_digit() || c == '-' => {
                let start = i;
                // Consume digits, decimal point, and exponent
                while i < chars.len() && (chars[i].is_ascii_digit()
                    || chars[i] == '.' || chars[i] == 'e' || chars[i] == 'E'
                    || chars[i] == '+' || chars[i] == '-')
                {
                    // - after E is part of exponent, otherwise stop
                    if chars[i] == '-' && i > start
                        && chars[i-1] != 'e' && chars[i-1] != 'E'
                    {
                        break;
                    }
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let n: f64 = num_str.parse().map_err(|_| format!("invalid number: {num_str}"))?;
                tokens.push(Token::Number(n));
            }

            // Keywords: true, false, null
            't' => {
                if chars[i..].starts_with(&['t', 'r', 'u', 'e']) {
                    tokens.push(Token::True);
                    i += 4;
                } else {
                    return Err(format!("unexpected token at position {i}"));
                }
            }
            'f' => {
                if chars[i..].starts_with(&['f', 'a', 'l', 's', 'e']) {
                    tokens.push(Token::False);
                    i += 5;
                } else {
                    return Err(format!("unexpected token at position {i}"));
                }
            }
            'n' => {
                if chars[i..].starts_with(&['n', 'u', 'l', 'l']) {
                    tokens.push(Token::Null);
                    i += 4;
                } else {
                    return Err(format!("unexpected token at position {i}"));
                }
            }

            c => return Err(format!("unexpected character '{c}' at position {i}")),
        }
    }

    Ok(tokens)
}

/// Parse a JSON value from tokens. Returns the value and remaining tokens.
fn parse_value(tokens: &[Token]) -> Result<(JsonValue, &[Token]), String> {
    match tokens.first() {
        None => Err("unexpected end of input".into()),

        Some(Token::OpenBrace) => {
            // Parse object: { key: value, key: value, ... }
            let mut rest = &tokens[1..]; // skip {
            let mut pairs = Vec::new();

            // Check for empty object
            match rest.first() {
                Some(Token::CloseBrace) => return Ok((JsonValue::Object(pairs), &rest[1..])),
                _ => {}
            }

            loop {
                // Each pair: String : Value
                match rest.first() {
                    Some(Token::String(key)) => {
                        rest = &rest[1..];
                        match rest.first() {
                            Some(Token::Colon) => { rest = &rest[1..]; }
                            other => return Err(format!("expected ':' after key, got {other:?}")),
                        }
                        let (val, remaining) = parse_value(rest)?;
                        pairs.push((key.clone(), val));
                        rest = remaining;
                    }
                    other => return Err(format!("expected string key, got {other:?}")),
                }

                // Next: either , or }
                match rest.first() {
                    Some(Token::Comma) => { rest = &rest[1..]; }
                    Some(Token::CloseBrace) => {
                        return Ok((JsonValue::Object(pairs), &rest[1..]));
                    }
                    other => return Err(format!("expected ',' or '}}', got {other:?}")),
                }
            }
        }

        Some(Token::OpenBracket) => {
            // Parse array: [value, value, ...]
            let mut rest = &tokens[1..]; // skip [
            let mut values = Vec::new();

            // Check for empty array
            match rest.first() {
                Some(Token::CloseBracket) => return Ok((JsonValue::Array(values), &rest[1..])),
                _ => {}
            }

            loop {
                let (val, remaining) = parse_value(rest)?;
                values.push(val);
                rest = remaining;

                match rest.first() {
                    Some(Token::Comma) => { rest = &rest[1..]; }
                    Some(Token::CloseBracket) => {
                        return Ok((JsonValue::Array(values), &rest[1..]));
                    }
                    other => return Err(format!("expected ',' or ']', got {other:?}")),
                }
            }
        }

        Some(Token::String(s)) => {
            let val = JsonValue::String(s.clone());
            Ok((val, &tokens[1..]))
        }

        Some(Token::Number(n)) => {
            let val = JsonValue::Number(*n);
            Ok((val, &tokens[1..]))
        }

        Some(Token::True) => Ok((JsonValue::Bool(true), &tokens[1..])),
        Some(Token::False) => Ok((JsonValue::Bool(false), &tokens[1..])),
        Some(Token::Null) => Ok((JsonValue::Null, &tokens[1..])),

        // Structural tokens aren't valid value starts on their own
        Some(other) => Err(format!("unexpected token {other:?}")),
    }
}

/// Query a JSON value by path. Path segments are dot-separated:
///   "a.b"    → obj["a"]["b"]
///   "items.1" → obj["items"][1]
///   ""        → root value
fn query<'a>(value: &'a JsonValue, path: &str) -> Option<&'a JsonValue> {
    if path.is_empty() {
        return Some(value);
    }

    // Split path by '.' and walk
    let segments: Vec<&str> = path.split('.').collect();
    let mut current = value;

    for segment in &segments {
        match current {
            JsonValue::Object(pairs) => {
                // Look up by string key — match to find it
                let found = pairs.iter().find(|(k, _)| k == segment);
                match found {
                    Some((_, v)) => current = v,
                    None => return None,
                }
            }
            JsonValue::Array(items) => {
                // Parse segment as index, match to access
                let idx: usize = segment.parse().ok()?;
                match items.get(idx) {
                    Some(v) => current = v,
                    None => return None,
                }
            }
            // Scalar values can't be further traversed
            _ => return None,
        }
    }

    Some(current)
}

/// Format a JsonValue into a compact string representation.
fn format_value(value: &JsonValue) -> String {
    match value {
        JsonValue::Object(pairs) => {
            let inner: Vec<String> = pairs
                .iter()
                .map(|(k, v)| format!("\"{k}\":{}", format_value(v)))
                .collect();
            format!("{{{}}}", inner.join(","))
        }
        JsonValue::Array(items) => {
            let inner: Vec<String> = items.iter().map(format_value).collect();
            format!("[{}]", inner.join(","))
        }
        JsonValue::String(s) => format!("\"{s}\""),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::Bool(true) => "true".into(),
        JsonValue::Bool(false) => "false".into(),
        JsonValue::Null => "null".into(),
    }
}

fn print_usage() {
    eprintln!(
        "Usage: json_query <json> [path]
        
A simple JSON path query tool.

Arguments:
  json     JSON string to parse (required)
  path     Dot-separated path to extract (optional)
           Examples: \"a.b\", \"items.0\", \"\"

Example:
  json_query '{{\"a\":{{\"b\":42}}}}' a.b
  => 42

  json_query '{{\"x\":[10,20,30]}}' x.1
  => 20

  json_query '{{\"x\":[10,20,30]}}'
  => {{\"x\":[10,20,30]}}"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            print_usage();
            process::exit(1);
        }
        2 => {
            let json_str = &args[1];
            if json_str == "--help" || json_str == "-h" {
                print_usage();
                return;
            }
            // Parse and print the whole JSON
            match lex(json_str) {
                Ok(tokens) => {
                    match parse_value(&tokens) {
                        Ok((value, remaining)) => {
                            if !remaining.is_empty() {
                                // Handle trailing content (shouldn't happen for valid single value)
                                // Just print what we parsed
                            }
                            println!("{}", format_value(&value));
                        }
                        Err(e) => {
                            eprintln!("Parse error: {e}");
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Lex error: {e}");
                    process::exit(1);
                }
            }
        }
        _ => {
            let json_str = &args[1];
            let path = &args[2];

            if json_str == "--help" || json_str == "-h" {
                print_usage();
                return;
            }

            match lex(json_str) {
                Ok(tokens) => {
                    match parse_value(&tokens) {
                        Ok((value, _remaining)) => {
                            match query(&value, path) {
                                Some(result) => println!("{}", format_value(result)),
                                None => {
                                    eprintln!("Path not found: {path}");
                                    process::exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Parse error: {e}");
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Lex error: {e}");
                    process::exit(1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_simple() {
        let tokens = lex(r#"{"a":42}"#).unwrap();
        assert_eq!(tokens, vec![
            Token::OpenBrace,
            Token::String("a".into()),
            Token::Colon,
            Token::Number(42.0),
            Token::CloseBrace,
        ]);
    }

    #[test]
    fn test_lex_array() {
        let tokens = lex("[1,2,3]").unwrap();
        assert_eq!(tokens, vec![
            Token::OpenBracket,
            Token::Number(1.0),
            Token::Comma,
            Token::Number(2.0),
            Token::Comma,
            Token::Number(3.0),
            Token::CloseBracket,
        ]);
    }

    #[test]
    fn test_lex_keywords() {
        let tokens = lex("[true,false,null]").unwrap();
        assert_eq!(tokens, vec![
            Token::OpenBracket,
            Token::True,
            Token::Comma,
            Token::False,
            Token::Comma,
            Token::Null,
            Token::CloseBracket,
        ]);
    }

    #[test]
    fn test_parse_object() {
        let tokens = lex(r#"{"a":1,"b":"hello"}"#).unwrap();
        let (val, remaining) = parse_value(&tokens).unwrap();
        assert!(remaining.is_empty());
        match val {
            JsonValue::Object(ref pairs) => {
                assert_eq!(pairs.len(), 2);
            }
            _ => panic!("expected object"),
        }
    }

    #[test]
    fn test_parse_nested() {
        let json = r#"{"a":{"b":42}}"#;
        let tokens = lex(json).unwrap();
        let (val, _) = parse_value(&tokens).unwrap();
        match &val {
            JsonValue::Object(pairs) => {
                let (_, inner) = &pairs[0];
                match inner {
                    JsonValue::Object(inner_pairs) => {
                        let (_, num) = &inner_pairs[0];
                        assert_eq!(num, &JsonValue::Number(42.0));
                    }
                    _ => panic!("expected inner object"),
                }
            }
            _ => panic!("expected outer object"),
        }
    }

    #[test]
    fn test_query_simple() {
        let json = r#"{"a":42,"b":"hello"}"#;
        let tokens = lex(json).unwrap();
        let (val, _) = parse_value(&tokens).unwrap();
        assert_eq!(query(&val, "a"), Some(&JsonValue::Number(42.0)));
        assert_eq!(query(&val, "b"), Some(&JsonValue::String("hello".into())));
        assert_eq!(query(&val, "c"), None);
    }

    #[test]
    fn test_query_nested() {
        let json = r#"{"a":{"b":42}}"#;
        let tokens = lex(json).unwrap();
        let (val, _) = parse_value(&tokens).unwrap();
        assert_eq!(query(&val, "a.b"), Some(&JsonValue::Number(42.0)));
    }

    #[test]
    fn test_query_array() {
        let json = r#"{"items":[10,20,30]}"#;
        let tokens = lex(json).unwrap();
        let (val, _) = parse_value(&tokens).unwrap();
        assert_eq!(query(&val, "items.0"), Some(&JsonValue::Number(10.0)));
        assert_eq!(query(&val, "items.1"), Some(&JsonValue::Number(20.0)));
        assert_eq!(query(&val, "items.2"), Some(&JsonValue::Number(30.0)));
        assert_eq!(query(&val, "items.3"), None);
    }

    #[test]
    fn test_query_root() {
        let json = r#"42"#;
        let tokens = lex(json).unwrap();
        let (val, _) = parse_value(&tokens).unwrap();
        assert_eq!(query(&val, ""), Some(&JsonValue::Number(42.0)));
    }

    #[test]
    fn test_format_roundtrip() {
        let json = r#"{"a":42,"b":"hello"}"#;
        let tokens = lex(json).unwrap();
        let (val, _) = parse_value(&tokens).unwrap();
        let formatted = format_value(&val);
        assert!(formatted.contains("\"a\":42"));
        assert!(formatted.contains("\"b\":\"hello\""));
    }
}

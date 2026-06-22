// ex02_parser.rs — Struct Parser that slices fields from input
//
// A Parser<'a> holds a reference to input text and provides methods
// that return slices of the original input. No allocations needed.
//
// TODO: Implement Parser with methods:
//   - parse_field(delimiter: char, field: usize) -> Option<&'a str>
//     Returns the Nth field (0-indexed) split by delimiter
//   - parse_line() -> Option<&'a str>
//     Returns the first line of input

struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input }
    }

    // Return the Nth field separated by the given delimiter
    fn parse_field(&self, delimiter: char, field: usize) -> Option<&'a str> {
        self.input.split(delimiter).nth(field)
    }

    // Return everything up to the first newline
    fn parse_line(&self) -> Option<&'a str> {
        self.input.lines().next()
    }
}

fn main() {
    let data = String::from("name:age:city\nAlice:30:New York\nBob:25:London");

    let parser = Parser::new(&data);

    // Parse first line
    if let Some(line) = parser.parse_line() {
        println!("First line: '{}'", line);
    }

    // Parse fields from the full input
    let fields = ["name", "age", "city"];
    for (i, name) in fields.iter().enumerate() {
        if let Some(val) = parser.parse_field(':', i) {
            println!("Field {} ({}): '{}'", i, name, val);
        }
    }

    // Parse "Alice:30:New York" line specifically
    let alice_data = "Alice:30:New York";
    let alice_parser = Parser::new(alice_data);
    println!("Alice's name: '{}'", alice_parser.parse_field(':', 0).unwrap());
    println!("Alice's age: '{}'", alice_parser.parse_field(':', 1).unwrap());
    println!("Alice's city: '{}'", alice_parser.parse_field(':', 2).unwrap());

    println!("\nAll slices borrow from the original input — no copies!");
}

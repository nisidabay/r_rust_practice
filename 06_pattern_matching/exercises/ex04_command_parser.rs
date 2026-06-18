// BONUS: ex04_command_parser.rs — Parse CLI commands like "add 5" "get 3"
// WHY: Real CLIs parse commands from strings — pattern matching on slices
//      is the idiomatic Rust way to do it.

#[derive(Debug, PartialEq)]
enum Command {
    Add(i32),    // "add 5" → Add(5)
    Get(usize), // "get 0" → Get(0) (zero-based index)
    Set { index: usize, value: i32 }, // "set 2 42" → Set { index: 2, value: 42 }
    Del(usize), // "del 1" → Del(1)
    Clear,      // "clear" → Clear
    Help,       // "help" → Help (show available commands)
    Quit,       // "quit" → Quit (exit)
}

/// Parse a single command string into a Command.
/// Returns None if the command is unrecognized or has invalid arguments.
fn parse_command(input: &str) -> Option<Command> {
    // TODO: Parse the input string into a Command.
    // Steps:
    //   1. Split on whitespace and collect into Vec<&str>
    //   2. Match on the slice: [cmd, args @ ..] pattern
    //   3. Match the command keyword in the first element
    //   4. Parse the arguments using the .. rest pattern
    //
    // Examples:
    //   "add 5"    => Command::Add(5)
    //   "get 0"    => Command::Get(0)
    //   "set 2 42" => Command::Set { index: 2, value: 42 }
    //   "del 1"    => Command::Del(1)
    //   "clear"    => Command::Clear
    //   "help"     => Command::Help
    //   "quit"     => Command::Quit
    //   "unknown"  => None
    //   "add"      => None (missing argument)
    //   "add x"    => None (non-numeric argument)
    //
    // Hints:
    //   - Use split_whitespace() not split(' ') to handle extra spaces
    //   - Pattern match on .as_slice() to handle variable-length args
    //   - str::parse::<i32>() and str::parse::<usize>() for numbers
    //   - For Set: match [cmd, idx, val] and parse both
    todo!("implement parse_command")
}

fn main() {
    let inputs = [
        "add 5",
        "get 0",
        "set 2 42",
        "del 1",
        "clear",
        "help",
        "quit",
        "add",
        "get abc",
        "bad_command",
        "set 2",
        "set x y",
    ];

    println!("=== Command Parser ===");
    for input in &inputs {
        match parse_command(input) {
            Some(cmd) => println!("{input:20} => {cmd:?}"),
            None => println!("{input:20} => invalid"),
        }
    }
}

// 01_stdin_stdout.rs — stdin, stdout, stderr
// Reading user input and printing to the terminal.
// Uses: std::io::stdin, println!, eprintln!, write!

use std::io::{self, Write};

fn main() -> io::Result<()> {
    // --- println! macro ---
    // Prints with a trailing newline to stdout.
    println!("Hello, world!");           // basic
    println!("Answer = {}", 42);         // positional formatting
    println!("{name} is {age}", name = "Alice", age = 30); // named args

    // --- print! macro ---
    // Prints WITHOUT a trailing newline. Must flush manually.
    print!("What is your name? ");
    io::stdout().flush()?; // flush ensures it shows before read_line

    // --- Reading a line from stdin ---
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim();              // remove trailing \n

    // --- eprintln! macro ---
    // Prints to stderr (error stream), useful for diagnostics / logs.
    eprintln!("[log] user entered name: '{}'", name);

    // --- writeln! macro ---
    // Writes formatted text into any io::Write implementor.
    let mut buf = Vec::new();
    writeln!(buf, "Name length: {}", name.len())?;
    // buf now contains: "Name length: 5\n" (for "Alice")
    println!("Log buffer: {:?}", String::from_utf8_lossy(&buf));

    // --- Reading multiple lines (until EOF / empty line) ---
    println!("Enter several lines (empty line to stop):");
    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        let bytes = io::stdin().read_line(&mut line)?;
        if bytes == 0 || line.trim().is_empty() {
            break;
        }
        lines.push(line.trim().to_string());
    }
    println!("You entered: {:?}", lines);

    Ok(())
}

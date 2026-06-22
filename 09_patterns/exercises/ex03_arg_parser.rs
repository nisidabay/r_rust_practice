// ex03_arg_parser.rs — Simple argv parser
// Parse "--name=value" and "--flag" patterns. Match literal flags.

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Skip program name (args[0])
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            // Boolean flags — present means true
            "--help" | "-h" => {
                println!("Usage: arg_parser [OPTIONS]");
                println!("  --name=VALUE   Set name");
                println!("  --count=N      Set count");
                println!("  --verbose      Enable verbose");
                println!("  --help         Show this help");
                return;
            }
            "--verbose" | "-v" => {
                println!("Verbose mode enabled");
            }
            s if s.starts_with("--") => {
                // Parse --key=value patterns
                if let Some((key, val)) = s.split_once('=') {
                    match key {
                        "--name" => println!("Name: {}", val),
                        "--count" => {
                            match val.parse::<u32>() {
                                Ok(n) => println!("Count: {}", n),
                                Err(_) => println!("Error: invalid count '{}'", val),
                            }
                        }
                        _ => println!("Unknown option: {}", key),
                    }
                } else {
                    println!("Unknown flag: {}", s);
                }
            }
            _ => {
                println!("Unexpected argument: {}", arg);
            }
        }
        i += 1;
    }

    if args.len() == 1 {
        println!("No arguments provided. Use --help for usage.");
    }
}

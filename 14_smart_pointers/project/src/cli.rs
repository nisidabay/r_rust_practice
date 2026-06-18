// cli.rs — Command-line argument parsing for todo_graph

use std::collections::HashMap;

/// Parsed CLI command and options.
pub struct Cli {
    pub command: String,
    pub args: Vec<String>,
    pub flags: HashMap<String, String>,
    pub raw_args: Vec<String>,
}

impl Cli {
    /// Parse command-line arguments. First argument is the command.
    pub fn parse(args: &[String]) -> Self {
        let mut command = String::new();
        let mut cmd_args = Vec::new();
        let mut flags = HashMap::new();
        let mut raw = Vec::new();

        if args.len() <= 1 {
            // No command — show help
            Self::print_help();
            std::process::exit(0);
        }

        // Check for --help first
        for arg in &args[1..] {
            if arg == "--help" || arg == "-h" {
                Self::print_help();
                std::process::exit(0);
            }
        }

        // First positional arg is the command
        command = args[1].clone();
        raw = args[1..].to_vec();

        // Parse remaining args
        let mut i = 2;
        while i < args.len() {
            let arg = &args[i];
            if arg.starts_with("--") {
                // Flag with potential value
                let flag = arg.trim_start_matches("--");
                if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                    flags.insert(flag.to_string(), args[i + 1].clone());
                    i += 2;
                } else {
                    flags.insert(flag.to_string(), String::new());
                    i += 1;
                }
            } else if arg.starts_with('-') && arg.len() == 2 {
                let flag = arg.trim_start_matches('-');
                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    flags.insert(flag.to_string(), args[i + 1].clone());
                    i += 2;
                } else {
                    flags.insert(flag.to_string(), String::new());
                    i += 1;
                }
            } else {
                cmd_args.push(arg.clone());
                i += 1;
            }
        }

        Cli {
            command,
            args: cmd_args,
            flags,
            raw_args: raw,
        }
    }

    fn print_help() {
        println!("todo_graph — A CLI todo manager with categories and dependencies");
        println!();
        println!("USAGE:");
        println!("  todo_graph <command> [args] [--flags]");
        println!();
        println!("COMMANDS:");
        println!("  new <title>    Create a new task");
        println!("    --cat <name>   Assign to a category (optional)");
        println!("    --priority N   Set priority (1-5, default 3)");
        println!("  list           List all tasks");
        println!("    --cat <name>   Filter by category");
        println!("    --status       Filter: todo, done, blocked");
        println!("  done <id>      Mark task as complete");
        println!("  depend <id> <dep_id>  Add dependency: task <id> depends on <dep_id>");
        println!("  blockers <id>  Show what tasks block <id>");
        println!("  categories     List all categories");
        println!("  --help         Show this help message");
        println!();
        println!("EXAMPLES:");
        println!("  todo_graph new \"Buy milk\" --cat shopping");
        println!("  todo_graph new \"Write report\" --cat work --priority 4");
        println!("  todo_graph list --cat work");
        println!("  todo_graph depend 2 1    # task 2 depends on task 1");
        println!("  todo_graph blockers 2    # show what blocks task 2");
        println!("  todo_graph done 1");
    }
}

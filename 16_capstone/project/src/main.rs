//! # Todo — CLI Entry Point
//!
//! Thin binary that delegates to the todo library.

use std::env;
use std::fs;

use todo::*;

// ============================================================
// CLI argument parsing (group 01 style)
// ============================================================

#[derive(Debug)]
pub struct CliConfig {
    pub command: String,
    pub args: Vec<String>,
    pub priority: Option<Priority>,
    pub tags: Vec<String>,
    pub due_date: Option<String>,
    pub format: Option<String>,
}

fn parse_cli_args() -> Result<CliConfig, TodoError> {
    let raw_args: Vec<String> = env::args().collect();
    if raw_args.len() < 2 {
        return Err(TodoError::InvalidCommand(
            "Usage: todo <command> [args] [flags]\n\nCommands:\n  add <text>     Add a new todo\n  list           List all todos\n  done <id>      Mark todo as done\n  remove <id>    Remove a todo\n  export         Export todos\n\nFlags:\n  --priority high|medium|low\n  --tag <tag>\n  --due <date>\n  --format csv|json\n\nExamples:\n  cargo run -- add \"Buy groceries\" --priority high --tag personal\n  cargo run -- list\n  cargo run -- done 1\n  cargo run -- export --format csv"
                .to_string(),
        ));
    }

    let command = raw_args[1].clone();
    let mut pos_args = Vec::new();
    let mut priority: Option<Priority> = None;
    let mut tags = Vec::new();
    let mut due_date: Option<String> = None;
    let mut format: Option<String> = None;

    let mut i = 2;
    while i < raw_args.len() {
        let arg = &raw_args[i];
        if arg == "--priority" {
            i += 1;
            if i < raw_args.len() {
                priority = Priority::from_str(&raw_args[i]);
                if priority.is_none() {
                    return Err(TodoError::Parse(format!(
                        "Invalid priority '{}'. Use: high, medium, or low",
                        raw_args[i]
                    )));
                }
            } else {
                return Err(TodoError::Parse("--priority requires a value".to_string()));
            }
        } else if arg == "--tag" {
            i += 1;
            if i < raw_args.len() {
                tags.push(raw_args[i].clone());
            } else {
                return Err(TodoError::Parse("--tag requires a value".to_string()));
            }
        } else if arg == "--due" {
            i += 1;
            if i < raw_args.len() {
                due_date = Some(raw_args[i].clone());
            } else {
                return Err(TodoError::Parse("--due requires a value".to_string()));
            }
        } else if arg == "--format" {
            i += 1;
            if i < raw_args.len() {
                let f = raw_args[i].to_lowercase();
                if f != "csv" && f != "json" {
                    return Err(TodoError::Parse(format!(
                        "Invalid format '{}'. Use: csv or json",
                        f
                    )));
                }
                format = Some(f);
            } else {
                return Err(TodoError::Parse("--format requires a value".to_string()));
            }
        } else if arg.starts_with("--") {
            return Err(TodoError::Parse(format!("Unknown flag: {}", arg)));
        } else {
            pos_args.push(arg.clone());
        }
        i += 1;
    }

    Ok(CliConfig {
        command,
        args: pos_args,
        priority,
        tags,
        due_date,
        format,
    })
}

// ============================================================
// Command handlers
// ============================================================

fn cmd_add(app: &mut App, config: &CliConfig) -> Result<(), TodoError> {
    let text = config
        .args
        .first()
        .ok_or_else(|| {
            TodoError::InvalidCommand(
                "Usage: todo add <text> [--priority high|medium|low] [--tag <tag>] [--due <date>]"
                    .to_string(),
            )
        })?
        .clone();

    let priority = config.priority.clone().unwrap_or(Priority::Medium);
    let tags = config.tags.clone();
    let due_date = config.due_date.clone();

    app.add(text, priority, tags, due_date);
    println!("Added todo #{}", app.next_id - 1);
    Ok(())
}

fn cmd_list(app: &App, _config: &CliConfig) -> Result<(), TodoError> {
    let display_config = DisplayConfig {
        show_index: false,
        color: false,
        prefix: "  ",
    };

    let all: Vec<&Todo> = app.todos.iter().collect();
    app.print_list(&all, "All Todos", &display_config);

    let pending_count = app.filter_by_status(&TodoStatus::Pending).count();
    let done_count = app.filter_by_status(&TodoStatus::Done).count();
    let high_count = app.filter_by_priority(&Priority::High).count();
    println!();
    println!(
        "Summary: {} total | {} pending | {} done | {} high priority",
        app.todos.len(),
        pending_count,
        done_count,
        high_count
    );

    Ok(())
}

fn cmd_done(app: &mut App, config: &CliConfig) -> Result<(), TodoError> {
    let id_str = config
        .args
        .first()
        .ok_or_else(|| TodoError::InvalidCommand("Usage: todo done <id>".to_string()))?;
    let id: u32 = id_str
        .parse()
        .map_err(|_| TodoError::Parse(format!("Invalid id: '{}'", id_str)))?;
    app.done(id)?;
    println!("Marked todo #{} as done", id);
    Ok(())
}

fn cmd_remove(app: &mut App, config: &CliConfig) -> Result<(), TodoError> {
    let id_str = config
        .args
        .first()
        .ok_or_else(|| TodoError::InvalidCommand("Usage: todo remove <id>".to_string()))?;
    let id: u32 = id_str
        .parse()
        .map_err(|_| TodoError::Parse(format!("Invalid id: '{}'", id_str)))?;
    app.remove(id)?;
    println!("Removed todo #{}", id);
    Ok(())
}

fn cmd_export(app: &App, config: &CliConfig) -> Result<(), TodoError> {
    let format = config.format.as_deref().unwrap_or("csv");
    let output = match format {
        "csv" => app.export_csv(),
        "json" => app.export_json(),
        _ => return Err(TodoError::Parse(format!("Unknown format: {}", format))),
    };

    let filename = format!("todos_export.{}", format);
    fs::write(&filename, &output)?;
    println!("Exported {} todos to '{}'", app.todos.len(), filename);
    println!("{}", output);
    Ok(())
}

// ============================================================
// Main — Load, process command, save (group 14 IO)
// ============================================================

fn main() {
    let data_path = "todos.json";

    // Load existing data or create new (group 14)
    let mut app = if fs::metadata(data_path).is_ok() {
        match App::load(data_path) {
            Ok(a) => {
                println!("Loaded {} todos from '{}'", a.todos.len(), data_path);
                a
            }
            Err(e) => {
                eprintln!("Warning: could not load '{}': {}", data_path, e);
                eprintln!("Starting with empty todo list");
                App::new(data_path)
            }
        }
    } else {
        println!("No existing data — starting fresh");
        App::new(data_path)
    };

    // Parse CLI args (group 01)
    let config = match parse_cli_args() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Process commands with match (group 09 style)
    let result = match config.command.as_str() {
        "add" => cmd_add(&mut app, &config),
        "list" => cmd_list(&app, &config),
        "done" => cmd_done(&mut app, &config),
        "remove" => cmd_remove(&mut app, &config),
        "export" => cmd_export(&app, &config),
        _ => Err(TodoError::InvalidCommand(format!(
            "Unknown command '{}'. Try: add, list, done, remove, export",
            config.command
        ))),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // Save at shutdown (group 14 IO — load/save)
    if app.dirty {
        match app.save() {
            Ok(()) => println!("Saved to '{}'", data_path),
            Err(e) => eprintln!("Warning: could not save: {}", e),
        }
    }
}

// A simple CLI task manager using an enum for TaskStatus and a struct for Task.
//
// Usage:
//   cargo run -- add "Buy groceries"
//   cargo run -- list
//   cargo run -- complete 1

use std::env;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum TaskStatus {
    Pending,
    InProgress,
    Done,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "pending"),
            TaskStatus::InProgress => write!(f, "in-progress"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    title: String,
    status: TaskStatus,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} — {}", self.id, self.title, self.status)
    }
}

struct TaskList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskList {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, title: String) {
        let task = Task {
            id: self.next_id,
            title,
            status: TaskStatus::Pending,
        };
        println!("Added: {task}");
        self.tasks.push(task);
        self.next_id += 1;
    }

    fn list(&self) {
        if self.tasks.is_empty() {
            println!("No tasks.");
            return;
        }
        for task in &self.tasks {
            println!("{task}");
        }
    }

    fn complete(&mut self, id: usize) -> Result<(), String> {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.status = TaskStatus::Done;
                println!("Completed: {task}");
                Ok(())
            }
            None => Err(format!("Task #{id} not found")),
        }
    }
}

fn print_usage() {
    eprintln!("Usage: task_list <add|list|complete> [args]");
    eprintln!("  add <title>      Add a new task");
    eprintln!("  list             List all tasks");
    eprintln!("  complete <id>    Mark a task as done");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    // In a real app we'd persist this; for the demo we use a local list.
    let mut list = TaskList::new();

    // Add some sample tasks so list/show is interesting.
    list.add(String::from("Learn Rust structs"));
    list.add(String::from("Learn enums"));
    list.add(String::from("Build a CLI tool"));

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Error: missing task title");
                print_usage();
                std::process::exit(1);
            }
            list.add(args[2..].join(" "));
        }
        "list" => {
            list.list();
        }
        "complete" => {
            if args.len() < 3 {
                eprintln!("Error: missing task id");
                print_usage();
                std::process::exit(1);
            }
            let id: usize = args[2]
                .parse()
                .unwrap_or_else(|_| {
                    eprintln!("Error: invalid id '{}'", args[2]);
                    std::process::exit(1);
                });
            match list.complete(id) {
                Ok(()) => {}
                Err(msg) => {
                    eprintln!("Error: {msg}");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Error: unknown command '{}'", args[1]);
            print_usage();
            std::process::exit(1);
        }
    }
}

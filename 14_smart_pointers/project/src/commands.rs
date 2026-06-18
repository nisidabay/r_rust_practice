// commands.rs — Command implementations for todo_graph

use crate::data::TodoStore;
use crate::cli::Cli;

/// Create a new task.
pub fn new_task(store: &mut TodoStore, cli: &Cli) {
    if cli.args.is_empty() {
        eprintln!("Error: missing task title. Usage: todo_graph new \"task title\" [--cat name]");
        std::process::exit(1);
    }

    let title = &cli.args[0];
    let category = cli.flags.get("cat").or_else(|| cli.flags.get("c"));
    let priority: u8 = cli
        .flags
        .get("priority")
        .and_then(|p| p.parse().ok())
        .unwrap_or(3);

    let id = store.add_task(title, category.map(|s| s.as_str()), priority);
    if let Some(cat) = category {
        println!("Created task #{} \"{}\" [{}] (priority {})", id, title, cat, priority);
    } else {
        println!("Created task #{} \"{}\" (priority {})", id, title, priority);
    }

    if let Err(e) = store.save() {
        eprintln!("Warning: could not save: {}", e);
    }
}

/// List tasks with optional filtering.
pub fn list_tasks(store: &TodoStore, cli: &Cli) {
    let category_filter = cli.flags.get("cat");
    let status_filter = cli.flags.get("status");

    let mut tasks: Vec<_> = store
        .tasks
        .iter()
        .filter(|task_rc| {
            let task = task_rc.borrow();
            // Filter by category
            if let Some(cat) = category_filter {
                if !store
                    .categories
                    .get(cat)
                    .map(|ids| ids.contains(&task.id))
                    .unwrap_or(false)
                {
                    return false;
                }
            }
            // Filter by status
            if let Some(status) = status_filter {
                match status.as_str() {
                    "done" if !task.done => return false,
                    "todo" if task.done => return false,
                    "blocked" if !task.is_blocked() => return false,
                    _ => {}
                }
            }
            true
        })
        .collect();

    // Sort by priority (highest first) then by id
    tasks.sort_by(|a, b| {
        let ta = a.borrow();
        let tb = b.borrow();
        tb.priority.cmp(&ta.priority).then(ta.id.cmp(&tb.id))
    });

    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    println!("Tasks:");
    println!("{:<4} {:<5} {:<30} {:<8} {}", "ID", "Done", "Title", "Priority", "Blocked?");
    println!("{}", "-".repeat(70));

    for task_rc in &tasks {
        let task = task_rc.borrow();
        let done_mark = if task.done { "✓" } else { " " };
        let blocked = if task.is_blocked() { "BLOCKED" } else { "" };
        println!(
            "{:<4} {:<5} {:<30} {:<8} {}",
            task.id, done_mark, task.title, task.priority, blocked
        );
    }
}

/// Mark a task as done.
pub fn mark_done(store: &mut TodoStore, cli: &Cli) {
    if cli.args.is_empty() {
        eprintln!("Error: missing task ID. Usage: todo_graph done <id>");
        std::process::exit(1);
    }

    let id: usize = match cli.args[0].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: invalid task ID '{}'", cli.args[0]);
            std::process::exit(1);
        }
    };

    match store.mark_done(id) {
        Ok(()) => {
            println!("Marked task #{} as done.", id);
            if let Err(e) = store.save() {
                eprintln!("Warning: could not save: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Add a dependency between tasks.
pub fn add_dependency(store: &mut TodoStore, cli: &Cli) {
    if cli.args.len() < 2 {
        eprintln!("Error: need task ID and dependency ID. Usage: todo_graph depend <id> <dep_id>");
        std::process::exit(1);
    }

    let id: usize = match cli.args[0].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: invalid task ID '{}'", cli.args[0]);
            std::process::exit(1);
        }
    };

    let dep_id: usize = match cli.args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: invalid dependency ID '{}'", cli.args[1]);
            std::process::exit(1);
        }
    };

    match store.add_dependency(id, dep_id) {
        Ok(()) => {
            println!("Task #{} now depends on task #{}.", id, dep_id);
            if let Err(e) = store.save() {
                eprintln!("Warning: could not save: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Show what blocks a task.
pub fn show_blockers(store: &TodoStore, cli: &Cli) {
    if cli.args.is_empty() {
        eprintln!("Error: missing task ID. Usage: todo_graph blockers <id>");
        std::process::exit(1);
    }

    let id: usize = match cli.args[0].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: invalid task ID '{}'", cli.args[0]);
            std::process::exit(1);
        }
    };

    let task = match store.get_task(id) {
        Some(t) => t,
        None => {
            eprintln!("Error: task #{} not found", id);
            std::process::exit(1);
        }
    };

    let task = task.borrow();
    let blockers = task.blockers();

    if blockers.is_empty() {
        println!("Task #{} \"{}\" has no blockers.", id, task.title);
        return;
    }

    println!("Task #{} \"{}\" is blocked by:", id, task.title);
    for blocker in &blockers {
        let b = blocker.borrow();
        println!("  #{} \"{}\" (priority {})", b.id, b.title, b.priority);
    }
}

/// List all categories.
pub fn list_categories(store: &TodoStore) {
    if store.categories.is_empty() {
        println!("No categories defined.");
        return;
    }

    println!("Categories:");
    for (name, ids) in &store.categories {
        let done_count = ids
            .iter()
            .filter(|id| store.get_task(**id).map(|t| t.borrow().done).unwrap_or(false))
            .count();
        println!("  {}: {} tasks ({} done)", name, ids.len(), done_count);
    }
}

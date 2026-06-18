// todo_graph — A CLI todo manager with categories and dependencies.
//
// Uses Rc<RefCell<>> for shared state between tasks and categories.
// A task can belong to multiple categories and have dependencies on
// other tasks. Both relationships use shared ownership via Rc, and
// interior mutability via RefCell allows modifying tasks after creation.
//
// Usage:
//   todo_graph new "Buy milk" --cat shopping
//   todo_graph list
//   todo_graph done 1
//   todo_graph depend 1 2   # task 1 depends on task 2
//   todo_graph blockers 1    # show what blocks task 1
//   todo_graph --help

mod cli;
mod data;
mod commands;

use cli::Cli;
use data::TodoStore;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cli = Cli::parse(&args);

    let mut store = TodoStore::load();
    
    match cli.command.as_str() {
        "new" => commands::new_task(&mut store, &cli),
        "list" => commands::list_tasks(&store, &cli),
        "done" => commands::mark_done(&mut store, &cli),
        "depend" => commands::add_dependency(&mut store, &cli),
        "blockers" => commands::show_blockers(&store, &cli),
        "categories" => commands::list_categories(&store),
        _ => {
            eprintln!("Unknown command: {}", cli.command);
            eprintln!("Run 'todo_graph --help' for usage.");
            std::process::exit(1);
        }
    }
}

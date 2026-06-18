// data.rs — Data model for todo_graph
//
// The core data model uses Rc<RefCell<Task>> so that:
// - Categories hold Rc handles to their tasks (shared ownership)
// - Task dependencies are Rc handles to other tasks (shared ownership)
// - RefCell allows marking tasks as done, changing priority, etc.
//   while the task is referenced from multiple places.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A task in the todo graph.
pub struct Task {
    pub id: usize,
    pub title: String,
    pub done: bool,
    pub priority: u8,
    pub dependencies: Vec<Rc<RefCell<Task>>>,
}

impl Task {
    pub fn new(id: usize, title: &str, priority: u8) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Task {
            id,
            title: title.to_string(),
            done: false,
            priority,
            dependencies: Vec::new(),
        }))
    }

    /// Returns true if this task is blocked (has incomplete dependencies).
    pub fn is_blocked(&self) -> bool {
        self.dependencies.iter().any(|dep| !dep.borrow().done)
    }

    /// Returns the list of incomplete dependencies (blockers).
    pub fn blockers(&self) -> Vec<Rc<RefCell<Task>>> {
        self.dependencies
            .iter()
            .filter(|dep| !dep.borrow().done)
            .cloned()
            .collect()
    }
}

/// The todo store: holds all tasks and categories.
pub struct TodoStore {
    pub tasks: Vec<Rc<RefCell<Task>>>,
    pub categories: HashMap<String, Vec<usize>>, // category name -> task ids
    next_id: usize,
    file_path: String,
}

impl TodoStore {
    pub fn new() -> Self {
        TodoStore {
            tasks: Vec::new(),
            categories: HashMap::new(),
            next_id: 1,
            file_path: String::new(),
        }
    }

    /// Load store from disk (simple JSON-ish save/load).
    /// Falls back to empty store if file doesn't exist or is corrupted.
    pub fn load() -> Self {
        let path = Self::default_path();
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        eprintln!("DEBUG: loading from {}, content len={}", path, content.len());
        eprintln!("DEBUG: content begins: {:?}", &content[..content.len().min(100)]);
        if content.is_empty() {
            eprintln!("DEBUG: empty content, returning new store");
            return TodoStore {
                file_path: path,
                ..Self::new()
            };
        }
        let store = Self::from_json(&content, path);
        eprintln!("DEBUG: loaded {} tasks, next_id={}", store.tasks.len(), store.next_id);
        store
    }

    /// Get a task by ID.
    pub fn get_task(&self, id: usize) -> Option<Rc<RefCell<Task>>> {
        self.tasks.iter().find(|t| t.borrow().id == id).cloned()
    }

    /// Add a new task and optionally assign it to a category.
    pub fn add_task(&mut self, title: &str, category: Option<&str>, priority: u8) -> usize {
        let task = Task::new(self.next_id, title, priority);
        let id = self.next_id;
        self.next_id += 1;

        if let Some(cat) = category {
            self.categories
                .entry(cat.to_string())
                .or_insert_with(Vec::new)
                .push(id);
        }

        self.tasks.push(task);
        id
    }

    /// Add a dependency: task `id` depends on `dep_id`.
    pub fn add_dependency(&mut self, id: usize, dep_id: usize) -> Result<(), String> {
        let task = self
            .get_task(id)
            .ok_or_else(|| format!("Task {} not found", id))?;
        let dep = self
            .get_task(dep_id)
            .ok_or_else(|| format!("Dependency task {} not found", dep_id))?;

        // Check for cycles: dep_id should not (transitively) depend on id
        if self.would_create_cycle(dep_id, id) {
            return Err(format!(
                "Cannot add dependency: creates a cycle ({} -> {} -> ... -> {})",
                id, dep_id, id
            ));
        }

        task.borrow_mut().dependencies.push(dep);
        Ok(())
    }

    /// Check if adding dep_id -> id would create a cycle.
    fn would_create_cycle(&self, from: usize, to: usize) -> bool {
        // BFS from `from` through its dependencies to see if we reach `to`
        let start = match self.get_task(from) {
            Some(t) => t,
            None => return false,
        };

        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            let current = current.borrow();
            if current.id == to {
                return true;
            }
            if visited.contains(&current.id) {
                continue;
            }
            visited.insert(current.id);
            for dep in &current.dependencies {
                queue.push_back(Rc::clone(dep));
            }
        }
        false
    }

    /// Mark a task as done.
    pub fn mark_done(&mut self, id: usize) -> Result<(), String> {
        let task = self
            .get_task(id)
            .ok_or_else(|| format!("Task {} not found", id))?;
        task.borrow_mut().done = true;
        Ok(())
    }

    /// Save store to disk.
    pub fn save(&self) -> std::io::Result<()> {
        let json = self.to_json();
        std::fs::write(&self.file_path, json)
    }

    /// Default storage path.
    fn default_path() -> String {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.todo_graph.json", home)
    }

    /// Simple JSON serialization (no serde dependency).
    fn to_json(&self) -> String {
        let mut tasks_json = Vec::new();
        for (i, task_rc) in self.tasks.iter().enumerate() {
            let task = task_rc.borrow();
            let deps: Vec<String> = task.dependencies.iter().map(|d| d.borrow().id.to_string()).collect();
            let deps_str = if deps.is_empty() {
                String::new()
            } else {
                format!(", \"deps\": [{}]", deps.join(", "))
            };
            let comma = if i < self.tasks.len() - 1 { "," } else { "" };
            tasks_json.push(format!(
                "    {{ \"id\": {}, \"title\": \"{}\", \"done\": {}, \"priority\": {}{} }}{}",
                task.id,
                task.title.replace('"', "\\\""),
                task.done,
                task.priority,
                deps_str,
                comma,
            ));
        }
        format!(
            "{{\n  \"tasks\": [\n{}\n  ],\n  \"next_id\": {}\n}}",
            tasks_json.join("\n"),
            self.next_id
        )
    }

    /// Simple JSON deserialization.
    fn from_json(content: &str, file_path: String) -> Self {
        let mut store = TodoStore {
            file_path,
            ..Self::new()
        };

        // Parse "next_id" first
        if let Some(val) = Self::extract_top_value(content, "next_id") {
            if let Ok(n) = val.parse::<usize>() {
                store.next_id = n;
            }
        }

        // Parse tasks: find "tasks" array and parse each object
        if let Some(tasks_start) = content.find("\"tasks\"") {
            let after = &content[tasks_start..];
            if let Some(arr_start) = after.find('[') {
                let after_arr = &after[arr_start + 1..];
                // Find matching closing bracket, counting nesting
                let mut depth = 1;
                let mut arr_end = 0;
                for (i, ch) in after_arr.char_indices() {
                    if ch == '[' { depth += 1; }
                    if ch == ']' { depth -= 1; }
                    if depth == 0 { arr_end = i; break; }
                }
                let tasks_content = &after_arr[..arr_end];
                // Parse each object in the array
                let mut obj_start = 0;
                loop {
                    // Find opening brace
                    let obj_begin = tasks_content[obj_start..].find('{');
                    if obj_begin.is_none() { break; }
                    let obj_begin = obj_start + obj_begin.unwrap();
                    let obj_rest = &tasks_content[obj_begin + 1..];
                    // Find matching closing brace
                    let mut depth = 1;
                    let mut obj_end = 0;
                    for (i, ch) in obj_rest.char_indices() {
                        if ch == '{' { depth += 1; }
                        if ch == '}' { depth -= 1; }
                        if depth == 0 { obj_end = i; break; }
                    }
                    let obj_str = &tasks_content[obj_begin..=obj_begin + 1 + obj_end];
                    if let Some(task) = Self::parse_task_line(obj_str) {
                        let task_rc = Task::new(task.0, &task.1, task.3);
                        task_rc.borrow_mut().done = task.2;
                        store.tasks.push(task_rc);
                    }
                    obj_start = obj_begin + 1 + obj_end + 1;
                }
            }
        }

        store
    }

    /// Extract a top-level JSON value for a field (simple, no nested parsing).
    fn extract_top_value(content: &str, field: &str) -> Option<String> {
        let pattern = format!("\"{}\": ", field);
        let start = content.find(&pattern)? + pattern.len();
        let rest = &content[start..];
        let trimmed = rest.trim_start();
        if trimmed.starts_with('"') {
            // String value
            let end = trimmed[1..].find('"')? + 2;
            Some(trimmed[1..end - 1].to_string())
        } else {
            // Number
            let end = trimmed.find(|c: char| c == ',' || c == '}' || c == '\n').unwrap_or(trimmed.len());
            Some(trimmed[..end].trim().to_string())
        }
    }

    /// Parse a single task JSON line: { "id": N, "title": "..", "done": bool, "priority": N, "deps": [..] }
    fn parse_task_line(line: &str) -> Option<(usize, String, bool, u8)> {
        let line = line.trim().trim_end_matches(',');
        if !line.starts_with('{') || !line.ends_with('}') {
            return None;
        }

        let id = Self::extract_field_value(line, "id")?.parse().ok()?;
        let title = Self::extract_field_value(line, "title")?;
        let title = title.trim_matches('"').to_string();
        let done = Self::extract_field_value(line, "done")? == "true";
        let priority: u8 = Self::extract_field_value(line, "priority")
            .and_then(|s| s.parse().ok())
            .unwrap_or(3);

        Some((id, title, done, priority))
    }

    /// Extract a field value from a simple JSON line.
    fn extract_field_value<'a>(line: &'a str, field: &str) -> Option<&'a str> {
        let pattern = format!("\"{}\": ", field);
        let start = line.find(&pattern)? + pattern.len();
        let rest = &line[start..];

        if rest.starts_with('"') {
            // String value
            let end = rest[1..].find('"')? + 2;
            Some(&rest[..end])
        } else {
            // Number or bool — find next comma or end
            let end = rest.find(|c: char| c == ',' || c == '}' || c == ']').unwrap_or(rest.len());
            Some(&rest[..end])
        }
    }
}

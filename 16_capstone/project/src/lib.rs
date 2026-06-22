//! # Todo — Library
//!
//! Shared types and logic for the todo CLI application.
//! Used by both main.rs (binary) and tests.

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io;

// ============================================================
// Custom Error (group 10 style)
// ============================================================

#[derive(Debug)]
pub enum TodoError {
    Io(io::Error),
    NotFound(String),
    Parse(String),
    InvalidCommand(String),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::Io(e) => write!(f, "I/O error: {}", e),
            TodoError::NotFound(s) => write!(f, "Not found: {}", s),
            TodoError::Parse(s) => write!(f, "Parse error: {}", s),
            TodoError::InvalidCommand(s) => write!(f, "Invalid command: {}", s),
        }
    }
}

impl std::error::Error for TodoError {}

impl From<io::Error> for TodoError {
    fn from(e: io::Error) -> Self {
        TodoError::Io(e)
    }
}

// ============================================================
// Enums (group 07 style)
// ============================================================

#[derive(Debug, Clone, PartialEq)]
pub enum TodoStatus {
    Pending,
    Done,
}

impl fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoStatus::Pending => write!(f, "Pending"),
            TodoStatus::Done => write!(f, "Done"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "high" => Some(Priority::High),
            "medium" => Some(Priority::Medium),
            "low" => Some(Priority::Low),
            _ => None,
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::High => write!(f, "high"),
            Priority::Medium => write!(f, "medium"),
            Priority::Low => write!(f, "low"),
        }
    }
}

// ============================================================
// Config with lifetime (group 12 style)
// ============================================================

#[derive(Debug)]
pub struct DisplayConfig<'a> {
    pub show_index: bool,
    pub color: bool,
    pub prefix: &'a str,
}

// ============================================================
// Struct Todo (group 06 style) with Display (group 11 style)
// ============================================================

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub status: TodoStatus,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub due_date: Option<String>,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_marker = match self.status {
            TodoStatus::Done => "[✓]",
            TodoStatus::Pending => "[ ]",
        };

        let priority_str = match self.priority {
            Priority::High => "❗high",
            Priority::Medium => "●med ",
            Priority::Low => "○low ",
        };

        let tags_str = if self.tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", self.tags.join(", "))
        };

        let due_str = match &self.due_date {
            Some(d) => format!(" (due: {})", d),
            None => String::new(),
        };

        write!(
            f,
            "{} {:>4} | {:<6} | {}{}{}",
            status_marker, self.id, priority_str, self.text, tags_str, due_str
        )
    }
}

// ============================================================
// The App struct (group 06 style)
// ============================================================

pub struct App {
    pub todos: Vec<Todo>,
    pub next_id: u32,
    pub file_path: String,
    /// HashMap lookup by tag (group 08 style)
    pub tag_index: HashMap<String, Vec<usize>>,
    pub dirty: bool,
}

impl App {
    pub fn new(file_path: &str) -> Self {
        App {
            todos: Vec::new(),
            next_id: 1,
            file_path: file_path.to_string(),
            tag_index: HashMap::new(),
            dirty: false,
        }
    }

    // ============================================================
    // Manual JSON-like serialization (no serde)
    // ============================================================

    pub fn to_json(&self) -> String {
        let mut s = String::new();
        s.push_str("{\n");
        s.push_str(&format!("  \"next_id\": {},\n", self.next_id));
        s.push_str("  \"todos\": [\n");
        for (i, t) in self.todos.iter().enumerate() {
            let comma = if i < self.todos.len() - 1 { "," } else { "" };
            let status_str = match t.status {
                TodoStatus::Pending => "\"Pending\"",
                TodoStatus::Done => "\"Done\"",
            };
            let priority_str = match t.priority {
                Priority::High => "\"high\"",
                Priority::Medium => "\"medium\"",
                Priority::Low => "\"low\"",
            };
            let tags_str: Vec<String> = t.tags.iter().map(|tag| format!("\"{}\"", tag)).collect();
            let due_str = match &t.due_date {
                Some(d) => format!("\"{}\"", d),
                None => "null".to_string(),
            };
            s.push_str(&format!(
                "    {{ \"id\": {}, \"text\": \"{}\", \"status\": {}, \"priority\": {}, \"tags\": [{}], \"due_date\": {} }}{}\n",
                t.id,
                t.text.replace('"', "\\\""),
                status_str,
                priority_str,
                tags_str.join(", "),
                due_str,
                comma
            ));
        }
        s.push_str("  ]\n");
        s.push_str("}\n");
        s
    }

    pub fn from_json(json: &str) -> Self {
        let mut app = App::new("todos.json");

        // Parse next_id
        for line in json.lines() {
            let t = line.trim();
            if t.starts_with("\"next_id\"") {
                if let Some(val) = t.split(':').nth(1) {
                    app.next_id = val.trim().trim_end_matches(',').parse().unwrap_or(1);
                }
                break;
            }
        }

        // Parse todos array
        let mut in_todos = false;
        for line in json.lines() {
            let t = line.trim();
            if t == "\"todos\": [" {
                in_todos = true;
                continue;
            }
            if in_todos {
                if t == "]" || t == "]," {
                    break;
                }
                // Simple manual parsing
                // Line: { "id": 1, "text": "Task 1", "status": "Done", ... }
                // split('"'): parts alternate key/value:
                //   0='{ ', 1='id', 2=': 1, ', 3='text', 4=': ', 5='Task 1',
                //   6=', ', 7='status', 8=': ', 9='Done',
                //   10=', ', 11='priority', 12=': ', 13='high',
                //   14=', ', 15='tags', 16=': [', 17='urgent', 18='], ',
                //   19='due_date', 20=': null },' 
                // OR: 16=': [], ', 17='due_date', 18=': ', 19='2025-06-01', 20=' }'
                let parts: Vec<&str> = t.split('"').collect();

                // Id: value at index 2 (after "id" key at 1)
                let id = parts.get(2).and_then(|s| s.trim().trim_end_matches(',').parse().ok()).unwrap_or(0);
                // Text: value at index 5
                let text = parts.get(5).unwrap_or(&"").to_string();
                // Status: value at index 9
                let status_val = *parts.get(9).unwrap_or(&"Pending");
                // Priority: value at index 13
                let priority_val = *parts.get(13).unwrap_or(&"medium");

                let status = if status_val == "Done" {
                    TodoStatus::Done
                } else {
                    TodoStatus::Pending
                };

                let priority = Priority::from_str(priority_val).unwrap_or(Priority::Medium);

                // Parse tags — they're at index 16+ (after priority)
                // Tags are in format: "tags": ["urgent"] or "tags": []
                let tags = if t.contains("\"tags\": [") {
                    let tag_part = parts.get(17).unwrap_or(&"");
                    let mut parsed_tags = Vec::new();
                    let trimmed = tag_part.trim();
                    if trimmed != "]" && trimmed != "]," {
                        for p in tag_part.split('"') {
                            let clean = p.trim().trim_end_matches(',').trim_end_matches(']').trim_end_matches('}').trim_end_matches(',');
                            if !clean.is_empty() && !clean.contains(':') && !clean.contains('[') && !clean.contains('{') && clean != "]" {
                                parsed_tags.push(clean.to_string());
                            }
                        }
                    }
                    parsed_tags
                } else {
                    Vec::new()
                };

                // Parse due_date — find it after "due_date" key
                // With tags: parts[19]="due_date", parts[20]=": ", parts[21]="2025-06-01" or "null"
                // Without tags: parts[17]="due_date", parts[18]=": null }"
                let due_idx = parts.iter().position(|&p| p.trim() == "due_date");
                let due_date = due_idx.and_then(|idx| {
                    let val_part = parts.get(idx + 2)?;
                    let val = val_part.trim().trim_end_matches(',').trim_end_matches('}').to_string();
                    if val.is_empty() || val == "null" { None } else { Some(val) }
                });

                app.todos.push(Todo {
                    id,
                    text,
                    status,
                    priority,
                    tags,
                    due_date,
                });
            }
        }

        app.build_index();
        app
    }

    // ============================================================
    // Index methods
    // ============================================================

    fn build_index(&mut self) {
        self.tag_index.clear();
        for (i, todo) in self.todos.iter().enumerate() {
            for tag in &todo.tags {
                let key = tag.to_lowercase();
                self.tag_index.entry(key).or_default().push(i);
            }
        }
    }

    // ============================================================
    // App methods — mutating
    // ============================================================

    pub fn add(&mut self, text: String, priority: Priority, tags: Vec<String>, due_date: Option<String>) {
        self.todos.push(Todo {
            id: self.next_id,
            text,
            status: TodoStatus::Pending,
            priority,
            tags,
            due_date,
        });
        self.next_id += 1;
        self.build_index();
        self.dirty = true;
    }

    pub fn done(&mut self, id: u32) -> Result<(), TodoError> {
        let todo = self
            .todos
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| TodoError::NotFound(format!("Todo with id {}", id)))?;
        todo.status = TodoStatus::Done;
        self.dirty = true;
        Ok(())
    }

    pub fn remove(&mut self, id: u32) -> Result<(), TodoError> {
        let pos = self
            .todos
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| TodoError::NotFound(format!("Todo with id {}", id)))?;
        self.todos.remove(pos);
        self.build_index();
        self.dirty = true;
        Ok(())
    }

    // ============================================================
    // Iterator methods for filtering (group 13 style)
    // ============================================================

    pub fn filter_by_status<'a>(&'a self, status: &'a TodoStatus) -> impl Iterator<Item = &'a Todo> {
        self.todos.iter().filter(move |t| t.status == *status)
    }

    pub fn filter_by_priority<'a>(&'a self, priority: &'a Priority) -> impl Iterator<Item = &'a Todo> {
        self.todos.iter().filter(move |t| t.priority == *priority)
    }

    /// HashMap lookup by tag (group 08)
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&Todo> {
        let key = tag.to_lowercase();
        self.tag_index
            .get(&key)
            .map(|indices| indices.iter().filter_map(|&i| self.todos.get(i)).collect())
            .unwrap_or_default()
    }

    pub fn search(&self, query: &str) -> impl Iterator<Item = &Todo> {
        let q = query.to_lowercase();
        self.todos.iter().filter(move |t| t.text.to_lowercase().contains(&q))
    }

    // ============================================================
    // Tabular output with alignment (group 02 formatting)
    // ============================================================

    pub fn print_list(&self, todos: &[&Todo], title: &str, _config: &DisplayConfig) {
        println!();
        println!("=== {} ===", title);
        if todos.is_empty() {
            println!("  (no items)");
            return;
        }
        println!(
            "  {:<3} {:<6} {:<8} {:<50} {}",
            "ID", "Status", "Priority", "Text", "Tags"
        );
        println!("  {}", "-".repeat(90));
        for todo in todos {
            let status_str = match todo.status {
                TodoStatus::Done => "Done   ",
                TodoStatus::Pending => "Pending",
            };
            let tags_str = if todo.tags.is_empty() {
                String::from("-")
            } else {
                todo.tags.join(", ")
            };
            let due_str = match &todo.due_date {
                Some(d) => format!(" (due: {})", d),
                None => String::new(),
            };
            println!(
                "  {:>3} {:<6} {:<8} {:<50} {}",
                todo.id,
                status_str,
                todo.priority,
                format!("{}{}", todo.text, due_str),
                tags_str,
            );
        }
        println!("  Total: {}", todos.len());
    }

    // ============================================================
    // Export
    // ============================================================

    pub fn export_csv(&self) -> String {
        let mut s = String::new();
        s.push_str("id,text,status,priority,tags,due_date\n");
        for todo in &self.todos {
            let tags_str = todo.tags.join(";");
            let due_str = todo.due_date.as_deref().unwrap_or("");
            s.push_str(&format!(
                "{},{},{},{},{},{}\n",
                todo.id,
                todo.text.replace(',', "\\,"),
                todo.status,
                todo.priority,
                tags_str,
                due_str
            ));
        }
        s
    }

    pub fn export_json(&self) -> String {
        self.to_json()
    }

    // ============================================================
    // File persistence (group 14 style)
    // ============================================================

    pub fn save(&self) -> Result<(), TodoError> {
        fs::write(&self.file_path, self.to_json())?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, TodoError> {
        let json = fs::read_to_string(path)?;
        Ok(App::from_json(&json))
    }
}

// 02_file_io.rs
// Capstone — Manual JSON-like file persistence (no serde)
//
// Concepts: fs::read_to_string, fs::write, manual serialization, struct state
//
// Usage:
//   rustc 02_file_io.rs && ./02_file_io

use std::fs;
use std::io;

#[derive(Debug)]
struct Note {
    id: u32,
    title: String,
    body: String,
}

#[derive(Debug)]
struct Notebook {
    notes: Vec<Note>,
    next_id: u32,
}

impl Notebook {
    fn new() -> Self {
        Notebook {
            notes: Vec::new(),
            next_id: 1,
        }
    }

    /// Manual JSON-like serialization (no serde crate)
    fn to_json(&self) -> String {
        let mut lines = Vec::new();
        lines.push("{".to_string());
        lines.push(format!("  \"next_id\": {},", self.next_id));
        lines.push("  \"notes\": [".to_string());
        for (i, note) in self.notes.iter().enumerate() {
            let comma = if i < self.notes.len() - 1 { "," } else { "" };
            lines.push(format!(
                "    {{ \"id\": {}, \"title\": \"{}\", \"body\": \"{}\" }}{}",
                note.id,
                note.title.replace('"', "\\\""),
                note.body.replace('"', "\\\""),
                comma
            ));
        }
        lines.push("  ]".to_string());
        lines.push("}".to_string());
        lines.join("\n")
    }

    /// Manual JSON-like deserialization
    fn from_json(json: &str) -> Self {
        let mut notebook = Notebook::new();

        // Parse next_id
        if let Some(line) = json.lines().find(|l| l.contains("\"next_id\"")) {
            if let Some(val) = line.split(':').nth(1) {
                notebook.next_id = val.trim().trim_end_matches(',').parse().unwrap_or(1);
            }
        }

        // Parse notes array
        let mut in_notes = false;
        for line in json.lines() {
            let trimmed = line.trim();
            if trimmed == "\"notes\": [" {
                in_notes = true;
                continue;
            }
            if in_notes {
                if trimmed == "]" || trimmed == "]," {
                    break;
                }
                if let Some(id_str) = trimmed.split('"').nth(3) {
                    if let Some(title) = trimmed.split('"').nth(7) {
                        if let Some(body) = trimmed.split('"').nth(11) {
                            notebook.notes.push(Note {
                                id: id_str.trim().parse().unwrap_or(0),
                                title: title.to_string(),
                                body: body.to_string(),
                            });
                        }
                    }
                }
            }
        }

        notebook
    }

    fn add(&mut self, title: &str, body: &str) {
        self.notes.push(Note {
            id: self.next_id,
            title: title.to_string(),
            body: body.to_string(),
        });
        self.next_id += 1;
    }

    fn save(&self, path: &str) -> io::Result<()> {
        fs::write(path, self.to_json())?;
        println!("Saved {} notes to '{}'", self.notes.len(), path);
        Ok(())
    }

    fn load(path: &str) -> io::Result<Self> {
        let json = fs::read_to_string(path)?;
        let notebook = Notebook::from_json(&json);
        println!("Loaded {} notes from '{}'", notebook.notes.len(), path);
        Ok(notebook)
    }
}

fn main() -> io::Result<()> {
    let path = "notebook_data.json";

    // Try to load existing, or create new
    let mut notebook = if fs::metadata(path).is_ok() {
        Notebook::load(path)?
    } else {
        println!("No existing data — creating new notebook");
        Notebook::new()
    };

    // Add some notes
    notebook.add("Grocery List", "Milk, eggs, bread");
    notebook.add("Meeting Notes", "Discuss Q3 roadmap with team");
    notebook.add("Ideas", "Rust capstone project plan");

    // Print current state
    println!();
    println!("=== Notebook Contents ===");
    for note in &notebook.notes {
        println!("  [{}] {} — {}", note.id, note.title, note.body);
    }
    println!();

    // Save to file
    notebook.save(path)?;

    // Show what we wrote
    println!();
    println!("=== Raw JSON Written ===");
    println!("{}", notebook.to_json());

    // Clean up
    fs::remove_file(path)?;
    println!();
    println!("Cleaned up — removed '{}'", path);

    Ok(())
}

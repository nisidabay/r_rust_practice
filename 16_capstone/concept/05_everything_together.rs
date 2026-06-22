// 05_everything_together.rs
// Capstone — Complete small contacts program combining all patterns
//
// Concepts: CLI args, file IO, error chaining, struct-app, Display, enums, HashMap, match
// No external crates — manual JSON-like persistence
//
// Usage:
//   rustc 05_everything_together.rs && ./05_everything_together

use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::io;

// --- Error type (group 10 style) ---

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    NotFound(String),
    Parse(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "I/O error: {}", e),
            AppError::NotFound(s) => write!(f, "Not found: {}", s),
            AppError::Parse(s) => write!(f, "Parse error: {}", s),
        }
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

// --- Enums (group 07 style) ---

#[derive(Debug, Clone, PartialEq)]
enum ContactGroup {
    Family,
    Friend,
    Work,
    Other(String),
}

impl fmt::Display for ContactGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContactGroup::Family => write!(f, "Family"),
            ContactGroup::Friend => write!(f, "Friend"),
            ContactGroup::Work => write!(f, "Work"),
            ContactGroup::Other(s) => write!(f, "{}", s),
        }
    }
}

// --- Struct (group 06 style) with Display (group 11 style) ---

#[derive(Debug, Clone)]
struct Contact {
    id: u32,
    name: String,
    phone: String,
    email: String,
    group: ContactGroup,
}

impl fmt::Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:>3}] {:<20} | {:<15} | {:<25} | {}",
            self.id, self.name, self.phone, self.email, self.group
        )
    }
}

// --- App struct (group 04/06 style) ---

struct ContactApp {
    contacts: Vec<Contact>,
    next_id: u32,
    file_path: String,
    group_index: HashMap<String, Vec<usize>>, // group 08 HashMap lookup
}

impl ContactApp {
    fn new(file_path: &str) -> Self {
        ContactApp {
            contacts: Vec::new(),
            next_id: 1,
            file_path: file_path.to_string(),
            group_index: HashMap::new(),
        }
    }

    // --- Manual JSON-like serialization (no serde) ---

    fn to_json(&self) -> String {
        let mut s = String::new();
        s.push_str("{\n");
        s.push_str(&format!("  \"next_id\": {},\n", self.next_id));
        s.push_str("  \"contacts\": [\n");
        for (i, c) in self.contacts.iter().enumerate() {
            let comma = if i < self.contacts.len() - 1 { "," } else { "" };
            let group_str = match &c.group {
                ContactGroup::Other(g) => format!("\"Other({})\"", g),
                g => format!("\"{}\"", g),
            };
            s.push_str(&format!(
                "    {{ \"id\": {}, \"name\": \"{}\", \"phone\": \"{}\", \"email\": \"{}\", \"group\": {} }}{}\n",
                c.id,
                c.name.replace('"', "\\\""),
                c.phone.replace('"', "\\\""),
                c.email.replace('"', "\\\""),
                group_str,
                comma
            ));
        }
        s.push_str("  ]\n");
        s.push_str("}\n");
        s
    }

    fn from_json(json: &str) -> Self {
        let mut app = ContactApp::new("contacts.json");

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

        // Parse contacts array
        let mut in_contacts = false;
        for line in json.lines() {
            let t = line.trim();
            if t == "\"contacts\": [" {
                in_contacts = true;
                continue;
            }
            if in_contacts {
                if t == "]" || t == "]," {
                    break;
                }
                // Extract fields via simple string splitting
                let id = t.split('"').nth(3).and_then(|s| s.trim().parse().ok()).unwrap_or(0);
                let name = t.split('"').nth(7).unwrap_or("").to_string();
                let phone = t.split('"').nth(11).unwrap_or("").to_string();
                let email = t.split('"').nth(15).unwrap_or("").to_string();
                let group_str = t.split('"').nth(19).unwrap_or("Other");

                let group = match group_str {
                    "Family" => ContactGroup::Family,
                    "Friend" => ContactGroup::Friend,
                    "Work" => ContactGroup::Work,
                    s => {
                        if let Some(inner) = s.strip_prefix("Other(") {
                            ContactGroup::Other(inner.trim_end_matches(')').to_string())
                        } else {
                            ContactGroup::Other(s.to_string())
                        }
                    }
                };

                app.contacts.push(Contact {
                    id,
                    name,
                    phone,
                    email,
                    group,
                });
            }
        }

        app.build_index();
        app
    }

    fn build_index(&mut self) {
        self.group_index.clear();
        for (i, contact) in self.contacts.iter().enumerate() {
            let key = contact.group.to_string().to_lowercase();
            self.group_index.entry(key).or_default().push(i);
        }
    }

    fn add(&mut self, name: &str, phone: &str, email: &str, group: ContactGroup) {
        self.contacts.push(Contact {
            id: self.next_id,
            name: name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
            group,
        });
        self.next_id += 1;
        self.build_index();
    }

    fn remove(&mut self, id: u32) -> Result<(), AppError> {
        let pos = self.contacts.iter().position(|c| c.id == id)
            .ok_or_else(|| AppError::NotFound(format!("contact id {}", id)))?;
        let removed = self.contacts.remove(pos);
        self.build_index();
        println!("Removed: {}", removed.name);
        Ok(())
    }

    /// Find contacts by group using HashMap (group 08)
    fn find_by_group(&self, group: &str) -> Vec<&Contact> {
        let key = group.to_lowercase();
        self.group_index
            .get(&key)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&i| self.contacts.get(i))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Iterator methods for filtering (group 13 style)
    fn search(&self, query: &str) -> impl Iterator<Item = &Contact> {
        let q = query.to_lowercase();
        self.contacts.iter().filter(move |c| {
            c.name.to_lowercase().contains(&q)
                || c.phone.contains(&q)
                || c.email.to_lowercase().contains(&q)
        })
    }

    fn save(&self) -> Result<(), AppError> {
        fs::write(&self.file_path, self.to_json())?;
        println!("Saved {} contacts to '{}'", self.contacts.len(), self.file_path);
        Ok(())
    }

    fn load(path: &str) -> Result<Self, AppError> {
        let json = fs::read_to_string(path)?;
        let app = ContactApp::from_json(&json);
        println!("Loaded {} contacts from '{}'", app.contacts.len(), path);
        Ok(app)
    }

    fn print_table(&self, contacts: &[&Contact], title: &str) {
        println!();
        println!("=== {} ===", title);
        if contacts.is_empty() {
            println!("  (no contacts)");
            return;
        }
        // Tabular output with alignment (group 02 style)
        println!("  {:<3} {:<20} {:<15} {:<25} {}", "ID", "Name", "Phone", "Email", "Group");
        println!("  {}", "-".repeat(80));
        for c in contacts {
            println!("  {}", c);
        }
        println!("  Total: {}", contacts.len());
    }
}

fn parse_group(s: &str) -> ContactGroup {
    match s.to_lowercase().as_str() {
        "family" => ContactGroup::Family,
        "friend" => ContactGroup::Friend,
        "work" => ContactGroup::Work,
        other => ContactGroup::Other(other.to_string()),
    }
}

fn print_usage() {
    println!("Usage: 05_everything_together <command> [args]");
    println!();
    println!("Commands:");
    println!("  add <name> <phone> <email> <group>    Add a contact");
    println!("  remove <id>                            Remove a contact");
    println!("  list                                  List all contacts");
    println!("  group <name>                          List contacts in group");
    println!("  search <query>                        Search contacts");
    println!("  interactive                           Start interactive mode");
    println!();
    println!("Groups: family, friend, work, or custom");
    println!();
    println!("Examples:");
    println!("  ./05_everything_together add Alice 555-0101 alice@example.com friend");
    println!("  ./05_everything_together list");
    println!("  ./05_everything_together search alice");
}

fn main() -> Result<(), AppError> {
    let path = "contacts_data.json";
    let mut app = if fs::metadata(path).is_ok() {
        ContactApp::load(path)?
    } else {
        let mut app = ContactApp::new(path);
        // Seed some sample data
        app.add("Alice Johnson", "555-0101", "alice@example.com", ContactGroup::Friend);
        app.add("Bob Smith", "555-0102", "bob@work.com", ContactGroup::Work);
        app.add("Charlie Brown", "555-0103", "charlie@family.org", ContactGroup::Family);
        app.add("Diana Prince", "555-0104", "diana@example.com", ContactGroup::Friend);
        app
    };

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 6 {
                return Err(AppError::Parse("Usage: add <name> <phone> <email> <group>".to_string()));
            }
            app.add(&args[2], &args[3], &args[4], parse_group(&args[5]));
            app.save()?;
        }
        "remove" => {
            if args.len() < 3 {
                return Err(AppError::Parse("Usage: remove <id>".to_string()));
            }
            let id: u32 = args[2].parse().map_err(|_| AppError::Parse("invalid id".to_string()))?;
            app.remove(id)?;
            app.save()?;
        }
        "list" => {
            let all: Vec<&Contact> = app.contacts.iter().collect();
            app.print_table(&all, "All Contacts");
        }
        "group" => {
            if args.len() < 3 {
                return Err(AppError::Parse("Usage: group <name>".to_string()));
            }
            let members = app.find_by_group(&args[2]);
            app.print_table(&members, &format!("Group: {}", args[2]));
        }
        "search" => {
            if args.len() < 3 {
                return Err(AppError::Parse("Usage: search <query>".to_string()));
            }
            let results: Vec<&Contact> = app.search(&args[2]).collect();
            app.print_table(&results, &format!("Search: \"{}\"", args[2]));
        }
        "interactive" => {
            println!("Interactive mode started. Type 'help' for commands.");
            loop {
                print!("> ");
                use std::io::Write;
                io::stdout().flush().unwrap();
                let mut input = String::new();
                if io::stdin().read_line(&mut input).is_err() || input.trim() == "quit" {
                    break;
                }
                let input = input.trim();
                if input.is_empty() { continue; }
                let parts: Vec<&str> = input.splitn(2, ' ').collect();
                match parts[0] {
                    "list" => {
                        let all: Vec<&Contact> = app.contacts.iter().collect();
                        app.print_table(&all, "All Contacts");
                    }
                    "help" => print_usage(),
                    other => println!("Unknown: {}. Type 'help'", other),
                }
            }
        }
        _ => {
            print_usage();
        }
    }

    // Clean up if we created the file
    if path == "contacts_data.json" && fs::metadata(path).is_ok() {
        fs::remove_file(path)?;
    }

    Ok(())
}

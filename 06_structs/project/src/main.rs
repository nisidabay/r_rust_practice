/*
 * Project: student gradebook
 *
 * Manages students and their grades using structs.
 * Commands: add <name>, grade <name> <score>, summary, best
 *
 * Usage:
 *   cargo run --
 *   > add Alice
 *   > grade Alice 85
 *   > summary
 *   > best
 *   > quit
 */

use std::io::{self, BufRead, Write};

struct Student {
    name: String,
    grades: Vec<f64>,
}

impl Student {
    fn new(name: &str) -> Self {
        Student {
            name: name.to_string(),
            grades: Vec::new(),
        }
    }

    fn add_grade(&mut self, score: f64) {
        self.grades.push(score);
    }

    fn average(&self) -> f64 {
        if self.grades.is_empty() {
            return 0.0;
        }
        self.grades.iter().sum::<f64>() / self.grades.len() as f64
    }

    fn best(&self) -> Option<f64> {
        self.grades.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap())
    }
}

fn main() {
    let mut students: Vec<Student> = Vec::new();
    let stdin = io::stdin();

    println!("Gradebook — Commands: add <name>, grade <name> <score>, summary, best, quit");
    print!("> ");
    io::stdout().flush().unwrap();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            print!("> ");
            io::stdout().flush().unwrap();
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "quit" | "exit" => break,
            "add" => {
                if parts.len() < 2 {
                    println!("Usage: add <name>");
                } else {
                    students.push(Student::new(parts[1]));
                    println!("Added {}", parts[1]);
                }
            }
            "grade" => {
                if parts.len() < 3 {
                    println!("Usage: grade <name> <score>");
                } else {
                    let score: f64 = match parts[2].parse() {
                        Ok(n) => n,
                        Err(_) => {
                            println!("Invalid score: {}", parts[2]);
                            print!("> ");
                            io::stdout().flush().unwrap();
                            continue;
                        }
                    };
                    if let Some(s) = students.iter_mut().find(|s| s.name == parts[1]) {
                        s.add_grade(score);
                        println!("{} scored {}", s.name, score);
                    } else {
                        println!("Student '{}' not found", parts[1]);
                    }
                }
            }
            "summary" => {
                if students.is_empty() {
                    println!("No students yet.");
                } else {
                    println!("{:<12} {:>8} {:>8} {:>8}", "Name", "Grades", "Avg", "Best");
                    println!("{}", "-".repeat(40));
                    for s in &students {
                        let best = s.best().map(|b| format!("{:.1}", b)).unwrap_or("-".into());
                        println!(
                            "{:<12} {:>3} {:>8.1} {:>8}",
                            s.name,
                            s.grades.len(),
                            s.average(),
                            best
                        );
                    }
                }
            }
            "best" => {
                let best = students
                    .iter()
                    .max_by(|a, b| a.average().partial_cmp(&b.average()).unwrap());
                match best {
                    Some(s) => println!("Best: {} (avg {:.1})", s.name, s.average()),
                    None => println!("No students yet."),
                }
            }
            _ => println!("Unknown: {}. Try: add, grade, summary, best, quit", parts[0]),
        }

        print!("> ");
        io::stdout().flush().unwrap();
    }
}

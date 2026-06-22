//! Integration tests for the todo CLI application.
//!
//! Tests serialization/deserialization, commands, filtering, and export.

use std::fs;

/// Re-export from main.rs for testing
/// We import via the crate since tests are integration tests
use todo::*;

#[test]
fn test_new_app_empty() {
    let app = App::new("test.json");
    assert_eq!(app.todos.len(), 0);
    assert_eq!(app.next_id, 1);
    assert!(app.tag_index.is_empty());
}

#[test]
fn test_add_todo() {
    let mut app = App::new("test.json");
    app.add("Test task".to_string(), Priority::High, vec!["urgent".to_string()], None);
    assert_eq!(app.todos.len(), 1);
    assert_eq!(app.todos[0].id, 1);
    assert_eq!(app.todos[0].text, "Test task");
    assert_eq!(app.todos[0].status, TodoStatus::Pending);
    assert_eq!(app.todos[0].priority, Priority::High);
    assert_eq!(app.todos[0].tags, vec!["urgent"]);
    assert!(app.todos[0].due_date.is_none());
}

#[test]
fn test_add_with_due_date() {
    let mut app = App::new("test.json");
    app.add(
        "Task with due date".to_string(),
        Priority::Low,
        vec![],
        Some("2025-12-31".to_string()),
    );
    assert_eq!(app.todos[0].due_date.as_deref(), Some("2025-12-31"));
    assert_eq!(app.todos[0].priority, Priority::Low);
}

#[test]
fn test_done_todo() {
    let mut app = App::new("test.json");
    app.add("Do something".to_string(), Priority::Medium, vec![], None);
    assert_eq!(app.todos[0].status, TodoStatus::Pending);
    app.done(1).unwrap();
    assert_eq!(app.todos[0].status, TodoStatus::Done);
}

#[test]
fn test_done_nonexistent() {
    let mut app = App::new("test.json");
    let result = app.done(42);
    assert!(result.is_err());
}

#[test]
fn test_remove_todo() {
    let mut app = App::new("test.json");
    app.add("First".to_string(), Priority::Medium, vec![], None);
    app.add("Second".to_string(), Priority::Medium, vec![], None);
    assert_eq!(app.todos.len(), 2);
    app.remove(1).unwrap();
    assert_eq!(app.todos.len(), 1);
    assert_eq!(app.todos[0].text, "Second");
}

#[test]
fn test_remove_nonexistent() {
    let mut app = App::new("test.json");
    let result = app.remove(99);
    assert!(result.is_err());
}

#[test]
fn test_filter_by_status() {
    let mut app = App::new("test.json");
    app.add("Task 1".to_string(), Priority::Medium, vec![], None);
    app.add("Task 2".to_string(), Priority::Medium, vec![], None);
    app.done(1).unwrap();

    let pending: Vec<&Todo> = app.filter_by_status(&TodoStatus::Pending).collect();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].id, 2);

    let done: Vec<&Todo> = app.filter_by_status(&TodoStatus::Done).collect();
    assert_eq!(done.len(), 1);
    assert_eq!(done[0].id, 1);
}

#[test]
fn test_filter_by_priority() {
    let mut app = App::new("test.json");
    app.add("High".to_string(), Priority::High, vec![], None);
    app.add("Medium".to_string(), Priority::Medium, vec![], None);
    app.add("Low".to_string(), Priority::Low, vec![], None);

    let high: Vec<&Todo> = app.filter_by_priority(&Priority::High).collect();
    assert_eq!(high.len(), 1);
    assert_eq!(high[0].text, "High");

    let all_medium: Vec<&Todo> = app.filter_by_priority(&Priority::Medium).collect();
    assert_eq!(all_medium.len(), 1);
}

#[test]
fn test_filter_by_tag() {
    let mut app = App::new("test.json");
    app.add("Work task".to_string(), Priority::High, vec!["work".to_string()], None);
    app.add("Personal".to_string(), Priority::Medium, vec!["personal".to_string()], None);
    app.add("Urgent work".to_string(), Priority::High, vec!["work".to_string(), "urgent".to_string()], None);

    let work_todos = app.filter_by_tag("work");
    assert_eq!(work_todos.len(), 2);

    let personal_todos = app.filter_by_tag("personal");
    assert_eq!(personal_todos.len(), 1);

    let nonexistent = app.filter_by_tag("nonexistent");
    assert_eq!(nonexistent.len(), 0);
}

#[test]
fn test_search() {
    let mut app = App::new("test.json");
    app.add("Buy groceries".to_string(), Priority::Medium, vec![], None);
    app.add("Buy milk".to_string(), Priority::Low, vec![], None);
    app.add("Read a book".to_string(), Priority::Medium, vec![], None);

    let results: Vec<&Todo> = app.search("buy").collect();
    assert_eq!(results.len(), 2);

    let results2: Vec<&Todo> = app.search("book").collect();
    assert_eq!(results2.len(), 1);
}

#[test]
fn test_json_roundtrip() {
    let mut app = App::new("test.json");
    app.add("Task 1".to_string(), Priority::High, vec!["urgent".to_string()], None);
    app.add("Task 2".to_string(), Priority::Low, vec![], Some("2025-06-01".to_string()));
    app.done(1).unwrap();

    let json = app.to_json();
    let loaded = App::from_json(&json);

    assert_eq!(loaded.todos.len(), 2);
    assert_eq!(loaded.todos[0].text, "Task 1");
    assert_eq!(loaded.todos[0].status, TodoStatus::Done);
    assert_eq!(loaded.todos[0].priority, Priority::High);
    assert_eq!(loaded.todos[0].tags, vec!["urgent"]);
    assert_eq!(loaded.todos[1].text, "Task 2");
    assert_eq!(loaded.todos[1].status, TodoStatus::Pending);
    assert_eq!(loaded.todos[1].priority, Priority::Low);
    assert_eq!(loaded.todos[1].due_date.as_deref(), Some("2025-06-01"));
}

#[test]
fn test_json_roundtrip_empty() {
    let app = App::new("test.json");
    let json = app.to_json();
    let loaded = App::from_json(&json);
    assert_eq!(loaded.todos.len(), 0);
    assert_eq!(loaded.next_id, 1);
}

#[test]
fn test_export_csv() {
    let mut app = App::new("test.json");
    app.add("Task A".to_string(), Priority::High, vec!["work".to_string()], None);
    app.add("Task B".to_string(), Priority::Low, vec![], Some("2025-07-01".to_string()));

    let csv = app.export_csv();
    assert!(csv.starts_with("id,text,status,priority,tags,due_date\n"));
    assert!(csv.contains("Task A"));
    assert!(csv.contains("Task B"));
    assert!(csv.contains("high"));
    assert!(csv.contains("low"));
}

#[test]
fn test_export_json() {
    let mut app = App::new("test.json");
    app.add("JSON task".to_string(), Priority::Medium, vec!["test".to_string()], None);

    let json_str = app.export_json();
    assert!(json_str.contains("\"todos\""));
    assert!(json_str.contains("JSON task"));
    assert!(json_str.contains("medium"));
    assert!(json_str.contains("test"));
}

#[test]
fn test_save_and_load() {
    let path = "/tmp/test_todos_save.json";
    let mut app = App::new(path);
    app.add("Saved task".to_string(), Priority::High, vec![], None);
    app.save().unwrap();

    assert!(fs::metadata(path).is_ok());

    let loaded = App::load(path).unwrap();
    assert_eq!(loaded.todos.len(), 1);
    assert_eq!(loaded.todos[0].text, "Saved task");

    fs::remove_file(path).unwrap();
}

#[test]
fn test_id_increment() {
    let mut app = App::new("test.json");
    app.add("First".to_string(), Priority::Medium, vec![], None);
    assert_eq!(app.next_id, 2);
    app.add("Second".to_string(), Priority::Medium, vec![], None);
    assert_eq!(app.next_id, 3);
}

#[test]
fn test_multiple_tags() {
    let mut app = App::new("test.json");
    app.add(
        "Multi-tag task".to_string(),
        Priority::Medium,
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        None,
    );
    assert_eq!(app.todos[0].tags.len(), 3);
    assert_eq!(app.tag_index.len(), 3);
}

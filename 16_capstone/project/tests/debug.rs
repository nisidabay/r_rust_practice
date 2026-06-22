#[cfg(test)]
mod debug_json {
    #[test]
    fn debug_parse() {
        use todo::*;
        let mut app = App::new("test.json");
        app.add(
            "Task 1".to_string(),
            Priority::High,
            vec!["urgent".to_string()],
            None,
        );
        app.add(
            "Task 2".to_string(),
            Priority::Low,
            vec![],
            Some("2025-06-01".to_string()),
        );
        app.done(1).unwrap();
        let json = app.to_json();
        eprintln!("=== JSON ===");
        eprintln!("{}", json);

        // Let's trace the parts
        for line in json.lines() {
            let t = line.trim();
            if t.starts_with("{ \"id\"") {
                let parts: Vec<&str> = t.split('"').collect();
                for (i, p) in parts.iter().enumerate() {
                    eprintln!("  parts[{}] = '{}'", i, p);
                }
            }
        }

        let loaded = App::from_json(&json);
        for t in &loaded.todos {
            eprintln!(
                "id={}, text='{}', status={:?}, priority={:?}, tags={:?}, due={:?}",
                t.id, t.text, t.status, t.priority, t.tags, t.due_date
            );
        }
        assert_eq!(loaded.todos.len(), 2);
        assert_eq!(loaded.todos[0].text, "Task 1");
        assert_eq!(loaded.todos[0].status, TodoStatus::Done);
        assert_eq!(loaded.todos[0].priority, Priority::High);
        assert_eq!(loaded.todos[0].tags, vec!["urgent"]);
    }
}

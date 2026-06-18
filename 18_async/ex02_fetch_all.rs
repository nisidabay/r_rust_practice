// ex02_fetch_all.rs
// Concurrently fetch multiple simulated data sources.
//
// In real async Rust, you'd use futures::future::join_all to
// await many futures at once. Here we demonstrate the concept
// with threads that run concurrently.

use std::thread;
use std::time::{Duration, Instant};

/// A simulated data source with a name and a delay.
struct DataSource {
    name: &'static str,
    delay_ms: u64,
}

impl DataSource {
    fn new(name: &'static str, delay_ms: u64) -> Self {
        Self { name, delay_ms }
    }

    /// "Fetch" data from this source. In real async, this would
    /// be an async fn that does non-blocking I/O.
    fn fetch(&self) -> String {
        thread::sleep(Duration::from_millis(self.delay_ms));
        format!("{}: {} bytes of data", self.name, self.delay_ms * 10)
    }
}

fn main() {
    println!("=== ex02: Concurrently fetching from multiple data sources ===\n");

    let sources = vec![
        DataSource::new("Users API", 800),
        DataSource::new("Orders API", 1200),
        DataSource::new("Products API", 500),
        DataSource::new("Inventory DB", 1500),
        DataSource::new("Analytics", 2000),
    ];

    let start = Instant::now();

    // Spawn all fetches concurrently (like futures::future::join_all)
    let handles: Vec<_> = sources
        .iter()
        .map(|source| {
            let name = source.name;
            let delay = source.delay_ms;
            thread::spawn(move || {
                let source = DataSource::new(name, delay);
                source.fetch()
            })
        })
        .collect();

    // Await all results
    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.join().unwrap();
        println!("  [{:?}] {result}", start.elapsed());
    }

    let total = start.elapsed();
    println!("\nAll {} sources fetched in {total:?}", sources.len());
    println!("(Sequential would take ~{}ms)", sources.iter().map(|s| s.delay_ms).sum::<u64>());

    assert!(
        total < Duration::from_millis(2100),
        "Concurrent fetch should complete in ~max(delay) time"
    );
    println!("✅ All data sources fetched concurrently!");
}

// 06_shared_state.rs — Arc<Mutex<HashMap>> for shared state across threads
//
// The one question: "How do you run code in parallel — safely?"
// Answer: Arc<Mutex<HashMap<K, V>>> for a shared, mutable dictionary across threads.
//
// This is a common pattern: a shared map (or any collection) protected by a Mutex,
// wrapped in Arc for multiple ownership. Each thread locks, reads/writes, and unlocks.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // --- Shared HashMap ---
    let shared_map: Arc<Mutex<HashMap<String, u64>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];

    // Spawn 5 threads, each inserting data into the shared map
    for id in 0..5 {
        let map = Arc::clone(&shared_map);
        handles.push(thread::spawn(move || {
            let mut guard = map.lock().unwrap();
            // Insert thread-specific data
            guard.insert(format!("thread_{id}"), id as u64 * 100);

            // Simulate some work while holding the lock
            guard.insert(format!("value_{id}"), (id as u64 + 1) * 10);

            // Lock released when guard is dropped at end of scope
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // Read the final state
    {
        let guard = shared_map.lock().unwrap();
        println!("Shared map contents ({len} entries):", len = guard.len());
        for (k, v) in guard.iter() {
            println!("  {k}: {v}");
        }
    }

    // --- More complex: shared counter with words ---
    let word_counts: Arc<Mutex<HashMap<String, u64>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let data_chunks = vec![
        vec!["hello", "world", "hello"],
        vec!["foo", "bar", "foo", "foo"],
        vec!["world", "baz"],
    ];

    let mut count_handles = vec![];

    for chunk in data_chunks {
        let map = Arc::clone(&word_counts);
        count_handles.push(thread::spawn(move || {
            let mut guard = map.lock().unwrap();
            for word in chunk {
                *guard.entry(word.to_string()).or_insert(0) += 1;
            }
            // Lock released here — other threads can proceed
        }));
    }

    for h in count_handles {
        h.join().unwrap();
    }

    // Print word counts
    {
        let guard = word_counts.lock().unwrap();
        let mut sorted: Vec<_> = guard.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1)); // sort by count descending
        println!("\nWord counts:");
        for (word, count) in sorted {
            println!("  {word}: {count}");
        }
    }

    // --- Fine-grained locking with multiple Mutexes ---
    // For better concurrency, you can split data into shards, each with its own Mutex
    struct ShardedMap {
        shards: Vec<Mutex<HashMap<String, usize>>>,
    }

    impl ShardedMap {
        fn new(num_shards: usize) -> Self {
            let mut shards = Vec::with_capacity(num_shards);
            for _ in 0..num_shards {
                shards.push(Mutex::new(HashMap::new()));
            }
            ShardedMap { shards }
        }

        fn shard_index(&self, key: &str) -> usize {
            // Simple hash-based sharding
            key.bytes().fold(0usize, |acc, b| acc.wrapping_add(b as usize))
                % self.shards.len()
        }

        fn insert(&self, key: String, value: usize) {
            let idx = self.shard_index(&key);
            let mut guard = self.shards[idx].lock().unwrap();
            guard.insert(key, value);
        }

        fn get(&self, key: &str) -> Option<usize> {
            let idx = self.shard_index(key);
            let guard = self.shards[idx].lock().unwrap();
            guard.get(key).copied()
        }
    }

    let sharded = Arc::new(ShardedMap::new(4));
    let mut shard_handles = vec![];

    for i in 0..8 {
        let map = Arc::clone(&sharded);
        shard_handles.push(thread::spawn(move || {
            map.insert(format!("key_{i}"), i * 10);
        }));
    }

    for h in shard_handles {
        h.join().unwrap();
    }

    for i in 0..8 {
        let key = format!("key_{i}");
        println!("{key}: {:?}", sharded.get(&key));
    }

    println!("All shared state examples done!");
}

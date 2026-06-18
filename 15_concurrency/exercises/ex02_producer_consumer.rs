// ex02_producer_consumer.rs — Multi-producer, single-consumer pipeline
//
// Task: Create N producer threads and 1 consumer thread connected by an mpsc
// channel. Each producer generates numbers and sends them. The consumer receives
// and sums them. Print the final sum.
//
// The one question: "How do you run code in parallel — safely?"
// Answer: Use mpsc channels. Clone the Sender for each producer.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Each producer sends `count` numbers starting from `start`, stepping by `step`.
fn producer(tx: mpsc::Sender<u64>, id: usize, start: u64, count: usize, step: u64) {
    for i in 0..count {
        let value = start + (i as u64) * step;
        tx.send(value).expect("Consumer dropped!");
        // Small delay to simulate work and allow interleaving
        thread::sleep(Duration::from_micros(100));
    }
    println!("Producer {id}: sent {count} values starting from {start}");
}

fn main() {
    let num_producers = 4;
    let values_per_producer = 25;

    let (tx, rx) = mpsc::channel::<u64>();

    let mut handles = vec![];

    // Spawn producers
    for id in 0..num_producers {
        let tx_clone = tx.clone();
        let start = (id as u64) * 100;
        handles.push(thread::spawn(move || {
            producer(tx_clone, id, start, values_per_producer, 1);
        }));
    }

    // Drop the original sender so the channel closes when all producers finish
    drop(tx);

    // Consumer: sum all received values
    let mut total: u64 = 0;
    let mut count = 0;
    for value in rx {
        total += value;
        count += 1;
    }

    println!("Consumer: received {count} values, total sum = {total}");

    // Wait for producers to finish
    for h in handles {
        h.join().unwrap();
    }

    // Verify
    let expected_first = 0u64; // Producer 0: 0..24
    let expected_last = 300u64 + (values_per_producer as u64 - 1); // Producer 3: 300..324
    // Each producer sends values_per_producer consecutive integers starting from start.
    // Sum for each = count * start + count*(count-1)/2
    let per_producer_sum = |start: u64| -> u64 {
        let c = values_per_producer as u64;
        c * start + c * (c - 1) / 2
    };
    let expected_total: u64 = (0..num_producers)
        .map(|id| per_producer_sum((id as u64) * 100))
        .sum();

    println!(
        "First value expected: {expected_first}, Last expected: {expected_last}"
    );
    println!(
        "Expected total: {expected_total}, Got: {total}",
    );
    assert_eq!(
        total, expected_total,
        "Total should be {expected_total}"
    );
    assert_eq!(count, num_producers * values_per_producer);
    println!("All producer-consumer tests passed!");
}

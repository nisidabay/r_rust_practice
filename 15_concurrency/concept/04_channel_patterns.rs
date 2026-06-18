// 04_channel_patterns.rs — Multiple producers, select-ish with try_recv
//
// The one question: "How do you run code in parallel — safely?"
// Answer: Use channel patterns like fan-out (many senders) and select loops
// with try_recv to handle multiple message sources.
//
// Rust's std doesn't have a native select! macro (that's in crossbeam or tokio),
// but we can approximate it with polling loops and try_recv.

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // --- Pattern 1: Fan-out (one receiver, many producers) ---
    println!("=== Pattern 1: Fan-out ===");
    fan_out_example();

    // --- Pattern 2: Fan-in (many receivers, one producer) via shared Receiver ---
    // Not possible with std mpsc — only one consumer. Use Arc<Mutex<Receiver>>
    // or switch to crossbeam. Shown here for understanding.
    println!("\n=== Pattern 2: Fan-in (work stealing) with Arc<Mutex<Receiver>> ===");
    fan_in_example();

    // --- Pattern 3: Polling multiple channels with try_recv ---
    println!("\n=== Pattern 3: Polling multiple channels ===");
    polling_multiple_example();

    // --- Pattern 4: Channel as a work queue (bounded via sync_channel) ---
    println!("\n=== Pattern 4: Sync channel (bounded) ===");
    sync_channel_example();

    // --- Pattern 5: Timeout on receive ---
    println!("\n=== Pattern 5: Recv with timeout ===");
    recv_with_timeout_example();

    println!("\nAll channel patterns done!");
}

// --- Pattern 1: Multiple producers, one consumer via cloned Senders ---
fn fan_out_example() {
    let (tx, rx) = mpsc::channel::<String>();

    let mut handles = vec![];
    for id in 0..5 {
        let tx_clone = tx.clone();
        handles.push(thread::spawn(move || {
            for i in 0..3 {
                tx_clone
                    .send(format!("Producer {id} message {i}"))
                    .unwrap();
                thread::sleep(Duration::from_millis(2));
            }
        }));
    }
    // Drop the original sender so the receiver loop knows when all senders are done
    drop(tx);

    // Collect all messages (loop ends when all Sender clones are dropped)
    let mut count = 0;
    for msg in rx {
        println!("{msg}");
        count += 1;
    }
    println!("Total messages received: {count}");

    for h in handles {
        h.join().unwrap();
    }
}

// --- Pattern 2: Multiple consumers, one producer via Arc<Mutex<Receiver>> ---
use std::sync::{Arc, Mutex};

fn fan_in_example() {
    // We wrap the Receiver in Arc<Mutex<>> so multiple threads can share it
    let (tx, rx) = mpsc::channel::<u64>();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];

    // Spawn 3 worker threads that share the receiver
    for id in 0..3 {
        let rx = Arc::clone(&rx);
        handles.push(thread::spawn(move || {
            loop {
                // Each worker locks the receiver, checks for a message
                let msg = {
                    let rx = rx.lock().unwrap();
                    rx.try_recv()
                };
                match msg {
                    Ok(val) => println!("Worker {id} got: {val}"),
                    Err(mpsc::TryRecvError::Empty) => {
                        // No message right now, yield and try again
                        thread::sleep(Duration::from_millis(1));
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        println!("Worker {id} sees channel closed, exiting");
                        break;
                    }
                }
            }
        }));
    }

    // Producer sends data
    for i in 0..10 {
        tx.send(i).unwrap();
        thread::sleep(Duration::from_millis(3));
    }
    drop(tx); // Close the channel

    for h in handles {
        h.join().unwrap();
    }
}

// --- Pattern 3: Polling multiple channels with try_recv ---
fn polling_multiple_example() {
    let (tx_a, rx_a) = mpsc::channel::<String>();
    let (tx_b, rx_b) = mpsc::channel::<String>();
    let (tx_c, rx_c) = mpsc::channel::<String>();

    // Start 3 threads, each sending on a different channel
    let h_a = thread::spawn(move || {
        thread::sleep(Duration::from_millis(3));
        tx_a.send("from channel A".into()).unwrap();
        "done_a"
    });
    let h_b = thread::spawn(move || {
        thread::sleep(Duration::from_millis(7));
        tx_b.send("from channel B".into()).unwrap();
        "done_b"
    });
    let h_c = thread::spawn(move || {
        thread::sleep(Duration::from_millis(1));
        tx_c.send("from channel C".into()).unwrap();
        "done_c"
    });

    // Poll all three channels with a timeout
    let start = Instant::now();
    let mut received = 0;
    while received < 3 && start.elapsed() < Duration::from_millis(100) {
        // Check each channel
        if let Ok(msg) = rx_a.try_recv() {
            println!("Got: {msg}");
            received += 1;
        }
        if let Ok(msg) = rx_b.try_recv() {
            println!("Got: {msg}");
            received += 1;
        }
        if let Ok(msg) = rx_c.try_recv() {
            println!("Got: {msg}");
            received += 1;
        }
        // Small sleep to avoid busy-waiting
        if received < 3 {
            thread::sleep(Duration::from_millis(1));
        }
    }

    h_a.join().unwrap();
    h_b.join().unwrap();
    h_c.join().unwrap();
}

// --- Pattern 4: sync_channel for bounded/blocking behavior ---
fn sync_channel_example() {
    // sync_channel(3) creates a channel that can hold at most 3 messages before
    // send() blocks (waits for receiver to consume)
    let (tx, rx) = mpsc::sync_channel::<i32>(3);

    let producer = thread::spawn(move || {
        for i in 0..10 {
            // This will block when the buffer (3) is full
            tx.send(i).unwrap();
            println!("Produced: {i}");
        }
    });

    // Slow consumer — reads one message at a time
    thread::sleep(Duration::from_millis(10));
    for msg in rx {
        println!("Consumed: {msg}");
        thread::sleep(Duration::from_millis(10));
    }

    producer.join().unwrap();
}

// --- Pattern 5: Recv with timeout using recv_timeout ---
use std::sync::mpsc::RecvTimeoutError;

fn recv_with_timeout_example() {
    // Note: recv_timeout is on std::sync::mpsc::Receiver
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        tx.send("delayed message".into()).unwrap();
    });

    // Try receiving with a 10ms timeout — will timeout because sender sleeps 20ms
    match rx.recv_timeout(Duration::from_millis(10)) {
        Ok(msg) => println!("Got: {msg}"),
        Err(RecvTimeoutError::Timeout) => println!("Timeout — no message yet"),
        Err(RecvTimeoutError::Disconnected) => println!("Sender disconnected"),
    }

    // Now wait with a longer timeout — should get the message
    match rx.recv_timeout(Duration::from_millis(50)) {
        Ok(msg) => println!("Got (after longer wait): {msg}"),
        Err(e) => println!("Error: {e:?}"),
    }
}

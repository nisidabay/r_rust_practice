// 03_message_passing.rs — mpsc channels, send/receive, Sender/Receiver
//
// The one question: "How do you run code in parallel — safely?"
// Answer: Use mpsc channels to pass messages between threads, avoiding shared state.
//
// mpsc = Multi-Producer, Single-Consumer. Many senders can send to one receiver.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // --- Creating a channel ---
    // channel() returns (Sender<T>, Receiver<T>)
    let (tx, rx) = mpsc::channel::<String>();

    // --- Sending from a thread ---
    let sender_handle = thread::spawn(move || {
        let msg = String::from("Hello from the thread!");
        // send() returns Result<(), SendError<T>> — Err if receiver dropped
        tx.send(msg).expect("Receiver dropped!");
        // After send, `msg` is moved into the channel; can't use it here
    });

    // --- Receiving on the main thread ---
    // recv() blocks until a message arrives, returns Result<T, RecvError>
    let received = rx.recv().expect("Sender dropped without sending!");
    println!("Got: {received}");

    sender_handle.join().unwrap();

    // --- Multiple messages, multiple producers ---
    let (tx1, rx1) = mpsc::channel::<u32>();

    // Clone the sender for multiple producers
    let tx2 = tx1.clone();

    let p1 = thread::spawn(move || {
        for i in 0..5 {
            tx1.send(i * 10).unwrap();
            thread::sleep(Duration::from_millis(5));
        }
    });

    let p2 = thread::spawn(move || {
        for i in 0..5 {
            tx2.send(i * 10 + 1).unwrap();
            thread::sleep(Duration::from_millis(5));
        }
    });

    // Receive all messages (both producers)
    for received in rx1 {
        println!("Received from channel: {received}");
    }
    // The loop ends when all senders (both tx clones) are dropped

    p1.join().unwrap();
    p2.join().unwrap();

    // --- try_recv — non-blocking receive ---
    let (tx3, rx3) = mpsc::channel::<i32>();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx3.send(42).unwrap();
    });

    // try_recv returns Err(Empty) if no message available
    let result = rx3.try_recv();
    match result {
        Ok(val) => println!("Got value immediately: {val}"),
        Err(mpsc::TryRecvError::Empty) => println!("No message yet (expected)"),
        Err(mpsc::TryRecvError::Disconnected) => println!("Sender dropped"),
    }

    // Wait for the actual message
    thread::sleep(Duration::from_millis(150));
    match rx3.try_recv() {
        Ok(val) => println!("Got value after waiting: {val}"),
        other => println!("Unexpected: {other:?}"),
    }

    // --- Sending multiple types via enum ---
    enum Message {
        Text(String),
        Number(i32),
        Quit,
    }

    let (tx4, rx4) = mpsc::channel::<Message>();

    thread::spawn(move || {
        tx4.send(Message::Text(String::from("hello"))).unwrap();
        tx4.send(Message::Number(99)).unwrap();
        tx4.send(Message::Quit).unwrap();
    });

    // Receive until Quit
    loop {
        match rx4.recv().unwrap() {
            Message::Text(t) => println!("Text message: {t}"),
            Message::Number(n) => println!("Number message: {n}"),
            Message::Quit => {
                println!("Quit message received, breaking");
                break;
            }
        }
    }

    println!("All message passing examples done!");
}

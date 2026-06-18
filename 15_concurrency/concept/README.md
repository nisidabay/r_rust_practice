# 15_concurrency — Threads, Channels, Arc, Mutex, Send/Sync, Atomics

**The one question:** "How do you run code in parallel — safely?"

**Answer:** Rust's ownership system + type system guarantees thread safety at compile time. The key primitives are:

## Primitives

| Primitive | Role | Why |
|-----------|------|-----|
| `thread::spawn` + `JoinHandle` | Spawn and join threads | Basic unit of parallelism |
| `move` closures | Transfer ownership into threads | Satisfy the borrow checker |
| `mpsc::channel` (Sender/Receiver) | Message passing between threads | Communicate without shared state |
| `Arc<T>` | Atomic reference counting | Share ownership across threads (multi-producer) |
| `Mutex<T>` | Mutual exclusion | Safely mutate shared data |
| `Arc<Mutex<T>>` | Shared mutable state | The "go-to" pattern for shared data |
| `Send`/`Sync` traits | Thread safety markers | Compiler-enforced at type level |
| `AtomicBool/AtomicU32/...` | Lock-free atomic operations | Fast concurrent counters and flags |
| `Barrier` | Synchronize threads at a point | Wait for N threads to arrive |
| `Condvar` | Condition variable | Block until a condition is met |
| `thread::scope` | Scoped threads (Rust 1.63+) | Borrow local data without Arc |
| `sync_channel` | Bounded channel | Backpressure with capacity limits |

## Key Patterns

1. **Message passing** — Use `mpsc::channel`; clone Sender for multi-producer
2. **Shared state** — Use `Arc<Mutex<T>>` for any shared mutable collection
3. **Scoped threads** — `thread::scope` for borrowing without Arc overhead
4. **Thread pool** — Reuse threads; avoid spawn/join overhead per task
5. **Atomics** — Fast counters, flags, and lock-free structures with `Ordering`

## Files

| # | File | Topic |
|---|------|-------|
| 01 | `01_thread_spawn.rs` | `thread::spawn`, `join`, basic threading |
| 02 | `02_move_closure.rs` | `move` closures with threads, ownership transfer |
| 03 | `03_message_passing.rs` | mpsc channels, send/receive, Sender/Receiver |
| 04 | `04_channel_patterns.rs` | Multiple producers, fan-in, sync_channel, try_recv, timeout |
| 05 | `05_mutex.rs` | `Arc<Mutex<T>>`, `lock()`, poisoning |
| 06 | `06_shared_state.rs` | `Arc<Mutex<HashMap>>` for shared state, sharding |
| 07 | `07_send_sync.rs` | `Send` and `Sync` traits — what they mean, what implements them |
| 08 | `08_thread_pool.rs` | Basic thread pool, scoped threads |
| 09 | `09_atomics.rs` | `AtomicBool`, `AtomicU32`, `Ordering`, CAS |
| 10 | `10_barrier_condvar.rs` | `Barrier` and `Condvar` for thread synchronization |

# 18_async — How do you handle many things at once — without threads?

## One Question
**How do you handle many things at once — without threads?**

Async Rust lets you juggle many I/O-bound tasks concurrently using a single thread. Instead of blocking the thread while waiting for data, async functions yield control back to an executor, which can poll thousands of futures in the time it takes one synchronous call to complete.

## Quick Start
```bash
# Run concept files that demonstrate async patterns
rustc --edition 2021 01_async_basics.rs && ./01_async_basics
rustc --edition 2021 02_block_on.rs && ./02_block_on
rustc --edition 2021 03_futures_intro.rs && ./03_futures_intro
rustc --edition 2021 04_async_chaining.rs && ./04_async_chaining
rustc --edition 2021 05_join_concurrent.rs && ./05_join_concurrent
rustc --edition 2021 06_async_vs_thread.rs && ./06_async_vs_thread

# Or build and run everything
make
```

## Learning Path
1. **01_async_basics.rs** — What `async fn` and `.await` actually do (no threads)
2. **02_block_on.rs** — A minimal executor: how futures get polled to completion
3. **03_futures_intro.rs** — The `Future` trait: what `poll` means and how state machines work
4. **04_async_chaining.rs** — Sequencing multiple async calls with `.await`
5. **05_join_concurrent.rs** — Running two futures concurrently with `join!`
6. **06_async_vs_thread.rs** — Tradeoffs: async vs OS threads, when to use which

## Common Patterns
| Pattern | When to Use |
|---|---|
| `.await` | Sequential async calls that depend on each other |
| `join!` / `join_all` | Fan-out: run many independent tasks |
| `select!` | Race: first one done wins |
| Custom executor | Embedded / no-std environments |
| `spawn` (via executor) | Fire-and-forget background work |
| Channel (async) | Pipeline: producer-consumer across tasks |

## Now Build Your Own
After studying the concepts, try the exercises:

1. **ex01_simulate_async.rs** — Simulate async I/O with `std::thread::sleep`
2. **ex02_fetch_all.rs** — Concurrently fetch multiple simulated data sources
3. **ex03_timeout.rs** — Race between a task and a timeout
4. **Bonus: ex04_retry.rs** — Retry with exponential backoff

Then build the **`fetch_multi`** project — a CLI tool that fetches multiple URLs concurrently.

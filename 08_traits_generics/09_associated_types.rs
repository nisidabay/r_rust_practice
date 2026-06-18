// 09_associated_types.rs
// Associated types in traits (Iterator pattern intro).
//
// An associated type is a placeholder type within a trait that each implementor
// sets once. Unlike generic parameters, there is only ONE associated type per
// impl. This is the key pattern behind std::iter::Iterator.
//
// Run: rustc --edition 2021 09_associated_types.rs && ./09_associated_types

// ── Basic trait with associated type ─────────────────────────────────────────
//
// The trait defines a Container that holds items of some type Item.
// Each implementor decides what Item is.

trait Container {
    /// The type of elements stored in this container.
    type Item;

    /// Insert an item into the container.
    fn insert(&mut self, item: Self::Item);

    /// Remove and return one item, if any.
    fn remove(&mut self) -> Option<Self::Item>;

    /// How many items are in the container?
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ── Implement for a stack (LIFO) ─────────────────────────────────────────────

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
}

impl<T> Container for Stack<T> {
    type Item = T;

    fn insert(&mut self, item: Self::Item) {
        self.items.push(item);
    }

    fn remove(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

// ── Implement for a queue (FIFO) ─────────────────────────────────────────────

struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { items: Vec::new() }
    }
}

impl<T> Container for Queue<T> {
    type Item = T;

    fn insert(&mut self, item: Self::Item) {
        self.items.push(item);
    }

    fn remove(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

// ── Associated types vs generic parameters ───────────────────────────────────
//
// Generic param: a trait can be implemented multiple times for the same type
//                (with different type params).
// Associated type: a trait can be implemented only ONCE per type.
//
// Example: with associated types, Counter can only have one Item type.
// With a generic param, Counter<u32> and Counter<String> could coexist.

struct Counter {
    count: u64,
}

/// A simple iterator using an associated type.
impl Iterator for Counter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}

// ── Trait with both associated types and generic parameters ──────────────────

/// A conversion trait that converts from an input type to an output type.
/// Associated type = output; generic parameter = input.
trait ConvertFrom<T> {
    type Output;
    fn convert(value: T) -> Self::Output;
}

#[derive(Debug, Clone, Copy)]
struct Kelvin(f64);
#[derive(Debug, Clone, Copy)]
struct Celsius(f64);

impl ConvertFrom<Celsius> for Kelvin {
    type Output = Self;
    fn convert(value: Celsius) -> Self::Output {
        Kelvin(value.0 + 273.15)
    }
}

impl ConvertFrom<Kelvin> for Celsius {
    type Output = Self;
    fn convert(value: Kelvin) -> Self::Output {
        Celsius(value.0 - 273.15)
    }
}

fn main() {
    // Container demo
    let mut stack: Stack<i32> = Stack::new();
    stack.insert(10);
    stack.insert(20);
    stack.insert(30);
    println!("Stack len = {}", stack.len());
    while let Some(item) = stack.remove() {
        print!("{item} ");
    }
    println!();

    let mut queue: Queue<&str> = Queue::new();
    queue.insert("first");
    queue.insert("second");
    queue.insert("third");
    println!("Queue len = {}", queue.len());
    while let Some(item) = queue.remove() {
        print!("{item} ");
    }
    println!();

    // Iterator demo
    let mut counter = Counter { count: 0 };
    println!("Counter next: {:?}", counter.next());
    println!("Counter next: {:?}", counter.next());
    println!("Counter next: {:?}", counter.next());

    // ConvertFrom demo
    let temp_c = Celsius(100.0);
    let temp_k: Kelvin = <Kelvin as ConvertFrom<Celsius>>::convert(temp_c);
    println!("100°C = {} K", temp_k.0);
    let back: Celsius = <Celsius as ConvertFrom<Kelvin>>::convert(temp_k);
    println!("{} K = {}°C", temp_k.0, back.0);
}

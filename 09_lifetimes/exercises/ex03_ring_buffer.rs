// ex03_ring_buffer.rs — Implement a ring buffer that tracks elements by reference.
//
// TODO: Implement a fixed-capacity ring buffer (circular buffer) that stores
// references to elements. The buffer should support push, pop, and iteration.
//
// This exercise demonstrates struct lifetimes with a mutable collection.

/// A fixed-capacity ring buffer that stores references to T.
///
/// The buffer wraps around when full: new elements overwrite the oldest ones.
/// Each element stored is a reference &'a T.
#[derive(Debug)]
struct RingBuffer<'a, T> {
    /// Fixed-size storage for references
    buffer: [Option<&'a T>; 4],
    /// Index where the next element will be written
    write_pos: usize,
    /// Number of elements currently in the buffer
    count: usize,
}

impl<'a, T> RingBuffer<'a, T> {
    /// Create a new empty ring buffer with capacity 4.
    fn new() -> Self {
        // Each element starts as None (no reference stored)
        RingBuffer {
            buffer: [None, None, None, None],
            write_pos: 0,
            count: 0,
        }
    }

    /// Push a reference to `value` into the buffer.
    /// If the buffer is full, overwrite the oldest element.
    ///
    /// TODO: Implement this.
    /// 1. Store the reference at `write_pos`
    /// 2. Advance `write_pos` (wrapping around at capacity)
    /// 3. Increment count (cap at capacity)
    fn push(&mut self, value: &'a T) {
        todo!("store the reference in the buffer")
    }

    /// Pop the oldest element from the buffer, if any.
    /// Returns None if the buffer is empty.
    ///
    /// TODO: Implement this.
    /// 1. Find the oldest element (count elements back from write_pos)
    /// 2. Remove it from the buffer
    /// 3. Decrement count
    fn pop(&mut self) -> Option<&'a T> {
        todo!("remove and return the oldest element")
    }

    /// Returns the number of elements in the buffer.
    fn len(&self) -> usize {
        self.count
    }

    /// Returns true if the buffer is empty.
    fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns true if the buffer is full.
    fn is_full(&self) -> bool {
        self.count == self.buffer.len()
    }

    /// Peek at the oldest element without removing it.
    fn peek(&self) -> Option<&'a T> {
        if self.count == 0 {
            return None;
        }
        let read_pos = if self.write_pos >= self.count {
            self.write_pos - self.count
        } else {
            self.buffer.len() + self.write_pos - self.count
        };
        self.buffer[read_pos].as_ref().copied()
    }
}

fn main() {
    // --- Test 1: basic push and pop ---
    let mut buf: RingBuffer<'_, i32> = RingBuffer::new();
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);

    let a = 10;
    let b = 20;
    let c = 30;

    buf.push(&a);
    assert!(!buf.is_empty());
    assert_eq!(buf.len(), 1);

    buf.push(&b);
    buf.push(&c);
    assert_eq!(buf.len(), 3);

    println!("Buffer after pushes: {:?}", buf);

    assert_eq!(buf.pop(), Some(&10));
    assert_eq!(buf.pop(), Some(&20));
    assert_eq!(buf.pop(), Some(&30));
    assert!(buf.is_empty());

    // --- Test 2: wrap-around ---
    let mut buf: RingBuffer<'_, i32> = RingBuffer::new();
    let vals = [1, 2, 3, 4, 5, 6];
    for v in &vals {
        buf.push(v);
    }
    // Overwritten: 1, 2 are gone. Buffer holds &3, &4, &5, &6
    assert_eq!(buf.len(), 4);
    assert!(buf.is_full());
    assert_eq!(buf.pop(), Some(&3));
    assert_eq!(buf.pop(), Some(&4));
    assert_eq!(buf.pop(), Some(&5));
    assert_eq!(buf.pop(), Some(&6));
    assert!(buf.is_empty());

    // --- Test 3: peek ---
    let mut buf: RingBuffer<'_, &str> = RingBuffer::new();
    let hello = "hello";
    let world = "world";
    buf.push(&hello);
    assert_eq!(buf.peek(), Some(&"hello"));
    buf.push(&world);
    assert_eq!(buf.peek(), Some(&"hello")); // oldest still "hello"
    buf.pop();
    assert_eq!(buf.peek(), Some(&"world"));

    // --- Test 4: push/pop alternating ---
    let mut buf: RingBuffer<'_, i32> = RingBuffer::new();
    let x = 42;
    let y = 99;
    buf.push(&x);
    assert_eq!(buf.pop(), Some(&42));
    buf.push(&y);
    assert_eq!(buf.pop(), Some(&99));
    assert!(buf.is_empty());

    // --- Test 5: pop from empty ---
    let mut buf: RingBuffer<'_, i32> = RingBuffer::new();
    assert_eq!(buf.pop(), None);

    println!("\n✓ ex03_ring_buffer passed!");
}

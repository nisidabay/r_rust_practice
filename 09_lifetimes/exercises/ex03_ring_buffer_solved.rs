// ex03_ring_buffer_solved.rs — Implement a ring buffer that tracks elements by reference.
//
// Solved version of ex03_ring_buffer.

/// A fixed-capacity ring buffer that stores references to T.
#[derive(Debug)]
struct RingBuffer<'a, T> {
    buffer: [Option<&'a T>; 4],
    write_pos: usize,
    count: usize,
}

impl<'a, T> RingBuffer<'a, T> {
    fn new() -> Self {
        RingBuffer {
            buffer: [None, None, None, None],
            write_pos: 0,
            count: 0,
        }
    }

    fn push(&mut self, value: &'a T) {
        self.buffer[self.write_pos] = Some(value);
        self.write_pos = (self.write_pos + 1) % self.buffer.len();
        if self.count < self.buffer.len() {
            self.count += 1;
        }
    }

    fn pop(&mut self) -> Option<&'a T> {
        if self.count == 0 {
            return None;
        }
        // The oldest element is `count` positions behind `write_pos`
        let read_pos = if self.write_pos >= self.count {
            self.write_pos - self.count
        } else {
            self.buffer.len() + self.write_pos - self.count
        };
        let value = self.buffer[read_pos].take();
        self.count -= 1;
        value
    }

    fn len(&self) -> usize {
        self.count
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn is_full(&self) -> bool {
        self.count == self.buffer.len()
    }

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
    assert_eq!(buf.peek(), Some(&"hello"));
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

    println!("\n✓ ex03_ring_buffer_solved passed!");
}

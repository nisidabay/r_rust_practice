// ex01_linked_list.rs — Singly linked list using Box (cons list pattern)
//
// Implement a singly linked list using Box for heap allocation.
// Each node either holds a value and points to the next node, or is empty.
// Box provides the indirection needed for this recursive type.

use std::fmt::Display;

// Define the List enum using Box for the recursive variant.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    /// Creates an empty list.
    fn new() -> Self {
        List::Nil
    }

    /// Adds a value at the front, returning a new list.
    /// Does NOT mutate — creates a new Cons node pointing to the old list.
    fn prepend(self, value: i32) -> Self {
        List::Cons(value, Box::new(self))
    }

    /// Returns the number of elements in the list.
    fn len(&self) -> usize {
        match self {
            List::Cons(_, rest) => 1 + rest.len(),
            List::Nil => 0,
        }
    }

    /// Returns the sum of all elements (0 for empty).
    fn sum(&self) -> i32 {
        match self {
            List::Cons(value, rest) => value + rest.sum(),
            List::Nil => 0,
        }
    }

    /// Collects all values into a Vec<i32>.
    fn to_vec(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = self;
        while let List::Cons(value, rest) = current {
            result.push(*value);
            current = rest;
        }
        result
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut current = self;
        while let List::Cons(value, rest) = current {
            write!(f, "{}", value)?;
            current = rest;
            if matches!(current, List::Cons(_, _)) {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

fn main() {
    // Test: build list and verify operations
    let list = List::new();

    assert_eq!(list.len(), 0);
    assert_eq!(list.sum(), 0);
    assert_eq!(list.to_vec(), vec![]);

    let list = list.prepend(3).prepend(2).prepend(1);

    assert_eq!(list.len(), 3);
    assert_eq!(list.sum(), 6);
    assert_eq!(list.to_vec(), vec![1, 2, 3]);

    println!("List: {}", list);
    println!("Length: {}", list.len());
    println!("Sum: {}", list.sum());
    println!("All tests passed!");
}

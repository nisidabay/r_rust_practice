// 04_rc_refcell.rs — Rc<RefCell<T>> pattern for shared mutable state
//
// Combining Rc<T> (shared ownership, multiple handles) with RefCell<T>
// (interior mutability, runtime borrow checking) gives us the ability
// to have multiple parts of our code own AND mutate the same data.
//
// This is the pattern used for graph structures, UI widget trees,
// and any scenario where you need shared mutable state without a
// single clear owner. It's single-threaded — use Arc<Mutex<T>> for
// multi-threaded equivalents.

use std::cell::RefCell;
use std::rc::Rc;

// A graph node that can be shared and mutated from multiple references.
// Each node has a value and a list of neighbors.
#[derive(Debug)]
struct GraphNode {
    value: String,
    neighbors: Vec<Rc<RefCell<GraphNode>>>,
}

impl GraphNode {
    fn new(value: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(GraphNode {
            value: value.to_string(),
            neighbors: Vec::new(),
        }))
    }

    fn add_edge(from: &Rc<RefCell<Self>>, to: &Rc<RefCell<Self>>) {
        from.borrow_mut().neighbors.push(Rc::clone(to));
        to.borrow_mut().neighbors.push(Rc::clone(from));
    }
}

fn main() {
    // Create three nodes in a triangle graph
    let node_a = GraphNode::new("A");
    let node_b = GraphNode::new("B");
    let node_c = GraphNode::new("C");

    // Connect them — both endpoints get Rc handles to each other
    GraphNode::add_edge(&node_a, &node_b);
    GraphNode::add_edge(&node_b, &node_c);
    GraphNode::add_edge(&node_c, &node_a);

    // Mutate node A through one reference
    node_a.borrow_mut().value = String::from("Alpha");

    // Read node A through a different reference (cloned Rc)
    let view = Rc::clone(&node_a);
    println!("Node A via clone: {}", view.borrow().value);
    assert_eq!(view.borrow().value, "Alpha");

    // Verify the triangle structure
    println!("\nTriangle graph:");
    println!("  {} has {} neighbors", node_a.borrow().value, node_a.borrow().neighbors.len());
    println!("  {} has {} neighbors", node_b.borrow().value, node_b.borrow().neighbors.len());
    println!("  {} has {} neighbors", node_c.borrow().value, node_c.borrow().neighbors.len());

    // Danger zone — avoiding reference cycles with Weak<T>
    // Rc<RefCell<T>> can create cycles that never free memory.
    // The solution: use Weak<RefCell<T>> for back-edges.
    demonstrate_cycle_safety();
}

/// Demonstrates how to avoid reference cycles using Weak.
fn demonstrate_cycle_safety() {
    use std::rc::Weak;

    struct Parent {
        name: String,
        children: Vec<Rc<RefCell<Child>>>,
    }

    struct Child {
        name: String,
        parent: Weak<RefCell<Parent>>, // Weak prevents cycles
    }

    let parent = Rc::new(RefCell::new(Parent {
        name: "Dad".to_string(),
        children: Vec::new(),
    }));

    let child = Rc::new(RefCell::new(Child {
        name: "Son".to_string(),
        parent: Rc::downgrade(&parent), // non-owning reference
    }));

    parent.borrow_mut().children.push(Rc::clone(&child));

    // Access parent through child's Weak reference
    if let Some(parent_ref) = child.borrow().parent.upgrade() {
        println!("\nChild's parent name: {}", parent_ref.borrow().name);
    }

    // When parent is dropped, the Weak reference becomes stale
    // (upgrade returns None) — no memory leak!
    println!("Parent strong count: {}", Rc::strong_count(&parent));
    println!("Parent weak count: {}", Rc::weak_count(&parent));
    // This drops cleanly because child holds a Weak<Parent>, not Rc
}

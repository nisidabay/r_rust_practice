// ex02_shared_graph.rs — Graph with shared nodes using Rc<RefCell<Node>>
//
// Build a simple directed graph where nodes can be shared (multiple
// incoming edges). Rc enables shared ownership, RefCell allows
// mutating node state after creation.

use std::cell::RefCell;
use std::rc::Rc;

// TODO: Define a Node struct with:
// - id: String
// - value: i32
// - edges: Vec<Rc<RefCell<Node>>>
//
// TODO: Implement:
// 1. Node::new(id, value) -> Rc<RefCell<Node>>
// 2. add_edge(from, to) -> adds direction from->to
// 3. count_reachable(start, max_depth) -> count nodes reachable within max_depth steps
// 4. find_by_value(start, target) -> Option<String> find first node id with matching value

fn main() {
    // Build a graph:
    //   A(10) -> B(20) -> C(30)
    //    |                 |
    //    +------> D(40) <--+
    //
    // Nodes are shared: C and A both point to D

    let a = TODO!(); // Node::new("A", 10);
    let b = TODO!(); // Node::new("B", 20);
    let c = TODO!(); // Node::new("C", 30);
    let d = TODO!(); // Node::new("D", 40);

    // Add edges
    // add_edge(&a, &b);
    // add_edge(&b, &c);
    // add_edge(&a, &d);
    // add_edge(&c, &d);

    // Tests
    // assert_eq!(count_reachable(&a, 0), 1);  // just A
    // assert_eq!(count_reachable(&a, 1), 3);  // A, B, D
    // assert_eq!(count_reachable(&a, 2), 4);  // A, B, C, D
    // assert_eq!(count_reachable(&a, 10), 4); // all reachable within 2

    // assert_eq!(find_by_value(&a, 40), Some(String::from("D")));
    // assert_eq!(find_by_value(&a, 999), None);

    // Verify mutation through shared reference
    // d.borrow_mut().value = 50;
    // assert_eq!(find_by_value(&a, 50), Some(String::from("D")));

    println!("Graph tests would pass when implemented!");
}

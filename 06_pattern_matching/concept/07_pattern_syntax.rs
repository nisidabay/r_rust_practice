// 07_pattern_syntax.rs — All pattern syntax in one file: ref, mut, @, |, .., _
// Learn: patterns aren't just literals — they have their own mini-language.

#[derive(Debug)]
enum Node {
    Leaf(i32),
    Branch { left: Box<Node>, right: Box<Node> },
}

fn main() {
    // --- | (or) — match one of several patterns ---
    let c = 'e';
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("{c} is a vowel"),
        _ => println!("{c} is a consonant"),
    }

    // --- .. (rest pattern) — skip remaining fields in tuples/structs/slices ---
    let triple = (1, 2, 3);
    match triple {
        (first, ..) => println!("first element: {first}"),
    }

    let arr = [10, 20, 30, 40, 50];
    match arr {
        // .. matches "the rest" — here it means elements at index 1, 2, 3
        [first, .., last] => println!("arr: first={first}, last={last}"),
    }

    // --- ref and ref mut — borrow instead of moving in a match arm ---
    let data = Some(String::from("hello"));

    // Without 'ref', match would move the String out of Some, consuming 'data'.
    match &data {
        // Pattern on a reference — automatically dereferences; s: &String
        Some(s) => println!("borrowed: {s} (len={})", s.len()),
        None => {} // data is still usable here after the match
    }
    println!("data still owned: {:?}", data); // compiles because we borrowed

    // --- @ (bind) — bind a value AND match a sub-pattern ---
    let num = 42;
    match num {
        // n @ 0..=100 means: match range, bind full value to n
        n @ 0..=100 => println!("{n} is between 0 and 100"),
        _ => println!("outside range"),
    }

    // --- @ on enum variants: bind the whole variant AND destructure ---
    let node = Node::Branch {
        left: Box::new(Node::Leaf(1)),
        right: Box::new(Node::Leaf(2)),
    };
    match node {
        n @ Node::Leaf(_) => println!("leaf node: {n:?}"),
        n @ Node::Branch { .. } => println!("branch node: {n:?}"),
    }

    // --- _ (wildcard) and .. together ---
    let complex = (1, 2, 3, 4, 5);
    match complex {
        (first, _, _, _, last) => println!("first={first}, last={last}"),
    }
    // Or more cleanly with ..
    match complex {
        (first, .., last) => println!("same: first={first}, last={last}"),
    }
}

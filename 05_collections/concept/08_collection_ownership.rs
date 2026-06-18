// 08_collection_ownership.rs — How ownership works with Vec and HashMap
//
// Collections own their elements. Moving them into or out of a collection
// transfers ownership. First 5 lines: move into Vec, move out, borrow,
// HashMap ownership, entry moves key.

use std::collections::HashMap;

fn main() {
    // 1. Moving data into a Vec — Vec owns its elements
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let mut v = Vec::new();
    v.push(s1); // s1 MOVED into v
    v.push(s2); // s2 MOVED into v
    // println!("{s1}"); // ERROR: s1 moved
    println!("v owns: {v:?}");

    // 2. Moving data out with pop
    if let Some(word) = v.pop() {
        println!("popped: {word}"); // word now owns the String
    }

    // 3. Borrowing from Vec — no move
    let first_word: &String = &v[0];
    println!("borrowed: {first_word}");
    println!("v still has it: {v:?}"); // v still valid

    // 4. HashMap takes ownership of keys and values
    let key = String::from("name");
    let val = String::from("Alice");
    let mut map = HashMap::new();
    map.insert(key, val); // both MOVED
    // println!("{key}"); // ERROR: key moved
    // println!("{val}"); // ERROR: val moved
    println!("map owns: {map:?}");

    // 5. get returns a reference — no move
    if let Some(name) = map.get("name") {
        println!("got ref: {name}"); // map still owns it
    }

    // 6. remove moves data *out* of the HashMap
    if let Some(taken) = map.remove("name") {
        println!("removed (now owned): {taken}"); // taken owns it now
    }

    // 7. Inserting a reference — careful with lifetimes
    let local = 42;
    let mut num_map: HashMap<&str, &i32> = HashMap::new();
    num_map.insert("answer", &local);
    println!("reference in map: {}", num_map["answer"]);
}

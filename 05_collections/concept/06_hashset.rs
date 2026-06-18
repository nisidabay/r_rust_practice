// 06_hashset.rs — HashSet, insert, contains, union, intersection
//
// HashSet<T> is a set of unique values — no duplicates, O(1) membership test.
// First 5 lines: create, insert, contains, union, intersection.

use std::collections::HashSet;

fn main() {
    // 1. HashSet::new — start empty
    let mut set = HashSet::new();

    // 2. insert — returns true if the value was *not* already present
    set.insert(1);
    set.insert(2);
    set.insert(3);
    let new = set.insert(2); // false — 2 already there
    println!("Was 2 new? {new}");

    // 3. contains — O(1) membership
    println!("Contains 3? {}", set.contains(&3));
    println!("Contains 99? {}", set.contains(&99));

    // 4. from Vec — dedup via collect
    let nums = vec![1, 2, 2, 3, 3, 3];
    let unique: HashSet<i32> = nums.into_iter().collect();
    println!("unique: {unique:?}"); // {1, 2, 3}

    // 5. union — elements in either set
    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5].into_iter().collect();
    let union: HashSet<_> = a.union(&b).copied().collect();
    println!("union: {union:?}");

    // 6. intersection — elements in both
    let intersection: HashSet<_> = a.intersection(&b).copied().collect();
    println!("intersection: {intersection:?}");

    // 7. difference / symmetric_difference
    let diff: HashSet<_> = a.difference(&b).copied().collect();
    println!("a - b: {diff:?}");

    // 8. remove / clear
    let mut s = HashSet::from([1, 2, 3]);
    s.remove(&2);
    println!("after remove: {s:?}");
    s.clear();
    println!("after clear: len={}", s.len());

    // 9. iter
    let colors = HashSet::from(["red", "green", "blue"]);
    for c in &colors {
        print!("{c} ");
    }
    println!();
}

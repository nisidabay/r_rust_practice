// ex02_merge_dedup.rs — Merge two sorted Vecs, remove duplicates
//
// Given two sorted Vec<i32>, merge them into one sorted Vec<i32>
// with no duplicates. Do NOT use HashSet for dedup — use slice logic.

fn merge_dedup(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            if result.last() != Some(&a[i]) {
                result.push(a[i]);
            }
            i += 1;
        } else if a[i] > b[j] {
            if result.last() != Some(&b[j]) {
                result.push(b[j]);
            }
            j += 1;
        } else {
            // equal — push once, advance both
            if result.last() != Some(&a[i]) {
                result.push(a[i]);
            }
            i += 1;
            j += 1;
        }
    }

    // Drain remaining from a
    while i < a.len() {
        if result.last() != Some(&a[i]) {
            result.push(a[i]);
        }
        i += 1;
    }

    // Drain remaining from b
    while j < b.len() {
        if result.last() != Some(&b[j]) {
            result.push(b[j]);
        }
        j += 1;
    }

    result
}

fn main() {
    let a = vec![1, 3, 5, 7, 9, 9];
    let b = vec![2, 3, 5, 6, 8, 10];

    let merged = merge_dedup(&a, &b);
    println!("merged: {merged:?}");

    assert_eq!(merged, vec![1, 2, 3, 5, 6, 7, 8, 9, 10]);

    // Edge cases
    assert_eq!(merge_dedup(&[], &[1, 2]), vec![1, 2]);
    assert_eq!(merge_dedup(&[1, 1, 1], &[1, 1, 1]), vec![1]);
    assert_eq!(merge_dedup(&[5], &[5]), vec![5]);

    println!("All assertions passed.");
}

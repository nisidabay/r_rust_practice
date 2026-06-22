/*
 * ex04_bonus_scoped.rs — Bonus Exercise
 *
 * Task: Write a working example that demonstrates using scoped blocks
 *       {} to avoid double borrow issues.
 *
 * Scenario: We want to compute the sum of a vec, then push a new element.
 * Without scoping, the immutable borrow from sum() blocks the mutable push().
 * Solution: scope the sum's borrow.
 *
 * Run: ./ex04_bonus_scoped
 * Expected: sum = 15, updated vec = [1, 2, 3, 4, 5, 6]
 */

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Compute the sum in a separate scope so the immutable borrow ends
    let sum = {
        let nums_ref = &numbers;       // immutable borrow starts
        let s: i32 = nums_ref.iter().sum(); // read through reference
        // nums_ref's borrow ends here (last use)
        s
    };

    println!("sum = {}", sum);

    // Now we can mutably borrow because the immutable borrow is done
    numbers.push(6);
    println!("updated vec = {:?}", numbers);

    // Another pattern: use a function that takes &[i32] (borrow)
    // and returns the sum (owned i32)

    let more_numbers = vec![10, 20, 30];
    let total = compute_sum(&more_numbers); // borrow
    // more_numbers still valid
    println!("total = {}, more_numbers = {:?}", total, more_numbers);
}

fn compute_sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

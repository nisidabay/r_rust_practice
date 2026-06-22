/*
 * ex01_swap.rs — Exercise 1
 *
 * Task: Write fn swap(a: &mut i32, b: &mut i32) that swaps two values.
 *       Then test it with a = 10, b = 20.
 *
 * Run: ./ex01_swap
 * Expected: After swap: a = 20, b = 10
 */

fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn main() {
    let mut a = 10;
    let mut b = 20;

    println!("Before swap: a = {}, b = {}", a, b);
    swap(&mut a, &mut b);
    println!("After swap:  a = {}, b = {}", a, b);

    assert_eq!(a, 20);
    assert_eq!(b, 10);
    println!("Test passed!");
}

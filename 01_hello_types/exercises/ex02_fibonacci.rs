// ex02_fibonacci — Print first N Fibonacci numbers
// Uses: let, mut, for loops, ranges, tuple assignment.
// The Fibonacci sequence: F(0)=0, F(1)=1, F(n)=F(n-1)+F(n-2).
// Run with: ./ex02_fibonacci <count>

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = if args.len() > 1 {
        args[1].parse().expect("Usage: ex02_fibonacci <count>")
    } else {
        10
    };

    // Print header.
    println!("First {n} Fibonacci numbers:");

    // Iterate from 0 to n-1, computing F(i) on the fly.
    // a holds F(i), b holds F(i+1) — tuple swap each iteration.
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for i in 0..n {
        if i > 0 {
            print!(", ");
        }
        print!("F({i})={a}");

        // Compute next pair: (a, b) = (b, a+b)
        // This is a single-statement tuple destructuring swap.
        (a, b) = (b, a + b);
    }
    println!();
}

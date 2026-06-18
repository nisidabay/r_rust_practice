// Collatz sequence: for a given starting n, repeatedly apply:
//   n → n/2      (if n is even)
//   n → 3n + 1   (if n is odd)
// Count how many steps until reaching 1 (the Collatz conjecture says
// this always terminates). We compute the sequence length for every
// starting number from 1 to 1000 and report the maximum.

fn collatz_length(mut n: u64) -> u64 {
    // Count steps until n reaches 1.
    // We use a while loop and pattern-match on the even/odd condition.
    let mut steps = 0;
    while n != 1 {
        // Check parity with a bitwise AND (faster than % 2).
        if n & 1 == 0 {
            // n is even — divide by 2.
            n /= 2;
        } else {
            // n is odd — apply 3n + 1.
            // Using saturating_mul to avoid overflow for very large sequences.
            n = n.saturating_mul(3).saturating_add(1);
        }
        steps += 1;
    }
    steps
}

fn main() {
    // Find the starting number ≤ 1000 that produces the longest Collatz chain.
    let mut max_steps = 0;
    let mut best_n = 0;

    // Iterate over all starting values and track the maximum.
    for n in 1..=1000 {
        let steps = collatz_length(n);
        if steps > max_steps {
            max_steps = steps;
            best_n = n;
        }
    }

    println!("Collatz sequence analysis (1..=1000):");
    println!("  Longest chain starts at: {best_n}");
    println!("  Number of steps: {max_steps}");

    // Also print a sample sequence for the best starting number.
    print!("  Sequence: {best_n}");
    let mut current = best_n as u64;
    while current != 1 {
        if current & 1 == 0 {
            current /= 2;
        } else {
            current = current.saturating_mul(3).saturating_add(1);
        }
        print!(" → {current}");
    }
    println!();
}

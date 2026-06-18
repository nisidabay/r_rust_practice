// ex04_bonus_pi — Approximate π using the Leibniz series
// π/4 = 1 - 1/3 + 1/5 - 1/7 + 1/9 - ...
// More terms = better approximation.
// Uses: f64 arithmetic, range with step_by, accumulating sum.
// Run with: ./ex04_bonus_pi <terms>

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let terms: u64 = if args.len() > 1 {
        args[1].parse().expect("Usage: ex04_bonus_pi <terms>")
    } else {
        1_000_000
    };

    // Leibniz sum: Σ (-1)^n / (2n+1) for n in 0..terms
    // We iterate odd denominators with step_by(2) to avoid recomputing 2n+1.
    let mut sum = 0.0_f64;
    let mut sign = 1.0_f64;

    // Range 1..2*terms, stepping by 2 gives denominators: 1, 3, 5, 7, ...
    for denom in (1..2 * terms).step_by(2) {
        sum += sign / (denom as f64);
        sign = -sign; // flip sign each term
    }

    let pi_approx = 4.0 * sum;
    println!("π ≈ {pi_approx:.15}  (Leibniz series, {terms} terms)");
    println!("π  = 3.141592653589793  (std f64 constant)");
    println!("error = {}", (pi_approx - std::f64::consts::PI).abs());
}

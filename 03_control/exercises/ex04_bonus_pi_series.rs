/*
 * ex04_bonus_pi_series.rs — Bonus Exercise
 *
 * Task: Approximate π using the Leibniz series:
 *       π/4 = 1 - 1/3 + 1/5 - 1/7 + 1/9 - ...
 *       Stop when term < 0.0001
 *
 * Run: ./ex04_bonus_pi_series
 * Expected: approximation of π (≈ 3.141...)
 */

fn main() {
    let mut pi_approx: f64 = 0.0;
    let mut sign: f64 = 1.0;
    let mut denominator: f64 = 1.0;
    let mut terms = 0;

    loop {
        let term = sign / denominator;
        pi_approx += term;
        terms += 1;

        if term.abs() < 0.0001 {
            break;
        }

        sign = -sign; // alternate sign
        denominator += 2.0;
    }

    println!("π ≈ {:.6}", pi_approx * 4.0);
    println!("Used {} terms, stopped at term {:.6}", terms, 1.0 / denominator);
    println!("Actual π = {:.6}", std::f64::consts::PI);
    println!("Error:    {:.6}", (std::f64::consts::PI - pi_approx * 4.0).abs());
}

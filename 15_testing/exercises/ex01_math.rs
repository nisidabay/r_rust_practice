// ex01_math.rs — fn factorial, fn gcd, fn lcm. Full test suite.
//
// Run: rustc --test --edition 2021 ex01_math.rs -o /tmp/test && /tmp/test

/// Compute factorial (n!)
pub fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => (2..=n).product(),
    }
}

/// Compute greatest common divisor (GCD) using Euclidean algorithm
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Compute least common multiple (LCM)
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- factorial tests ---
    #[test]
    fn factorial_zero() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_one() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn factorial_small() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn factorial_large() {
        assert_eq!(factorial(10), 3_628_800);
    }

    #[test]
    fn factorial_two() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_three() {
        assert_eq!(factorial(3), 6);
    }

    // --- gcd tests ---
    #[test]
    fn gcd_of_same() {
        assert_eq!(gcd(10, 10), 10);
    }

    #[test]
    fn gcd_coprime() {
        assert_eq!(gcd(7, 13), 1);
    }

    #[test]
    fn gcd_common() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(54, 24), 6);
    }

    #[test]
    fn gcd_with_zero() {
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
    }

    #[test]
    fn gcd_both_zero() {
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn gcd_large() {
        assert_eq!(gcd(123456, 7890), 6);
    }

    // --- lcm tests ---
    #[test]
    fn lcm_simple() {
        assert_eq!(lcm(4, 6), 12);
    }

    #[test]
    fn lcm_coprime() {
        assert_eq!(lcm(3, 5), 15);
    }

    #[test]
    fn lcm_same() {
        assert_eq!(lcm(7, 7), 7);
    }

    #[test]
    fn lcm_with_zero() {
        assert_eq!(lcm(0, 5), 0);
        assert_eq!(lcm(5, 0), 0);
    }

    #[test]
    fn lcm_one_is_lcm() {
        assert_eq!(lcm(1, 12), 12);
    }

    #[test]
    fn lcm_large() {
        assert_eq!(lcm(12, 18), 36);
    }

    // --- property-based tests ---
    #[test]
    fn gcd_commutative() {
        for a in 1..=50u64 {
            for b in 1..=50u64 {
                assert_eq!(gcd(a, b), gcd(b, a));
            }
        }
    }

    #[test]
    fn lcm_mul_eq_gcd_mul_lcm() {
        // a * b == gcd(a,b) * lcm(a,b)
        let pairs = [(4, 6), (12, 18), (7, 13), (10, 25), (8, 12)];
        for &(a, b) in &pairs {
            assert_eq!(a * b, gcd(a, b) * lcm(a, b));
        }
    }
}

fn main() {
    println!("factorial(5) = {}", factorial(5));
    println!("gcd(12, 8) = {}", gcd(12, 8));
    println!("lcm(4, 6) = {}", lcm(4, 6));
    println!("\nRun: rustc --test --edition 2021 ex01_math.rs -o /tmp/t && /tmp/t");
}

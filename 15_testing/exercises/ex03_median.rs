// ex03_median.rs — fn median(v: &[i32]) -> Option<i32>
//
// Tests: odd, even, empty, single, negative
//
// Run: rustc --test --edition 2021 ex03_median.rs -o /tmp/test && /tmp/test

/// Compute the median of a sorted slice of i32 values.
/// Returns None for empty slices.
/// For even-length slices, returns the lower middle value.
pub fn median(v: &[i32]) -> Option<i32> {
    if v.is_empty() {
        return None;
    }

    let mut sorted = v.to_vec();
    sorted.sort();
    let len = sorted.len();

    if len % 2 == 1 {
        // Odd length — middle element
        Some(sorted[len / 2])
    } else {
        // Even length — lower middle
        Some(sorted[len / 2 - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_length() {
        assert_eq!(median(&[1, 2, 3, 4, 5]), Some(3));
    }

    #[test]
    fn odd_length_unsorted() {
        assert_eq!(median(&[5, 2, 1, 4, 3]), Some(3));
    }

    #[test]
    fn even_length() {
        // [1, 2, 3, 4] -> lower middle is 2
        assert_eq!(median(&[1, 2, 3, 4]), Some(2));
    }

    #[test]
    fn even_length_unsorted() {
        assert_eq!(median(&[4, 1, 3, 2]), Some(2));
    }

    #[test]
    fn empty_slice() {
        let v: [i32; 0] = [];
        assert_eq!(median(&v), None);
    }

    #[test]
    fn single_element() {
        assert_eq!(median(&[42]), Some(42));
    }

    #[test]
    fn two_elements() {
        assert_eq!(median(&[10, 20]), Some(10));
    }

    #[test]
    fn negative_odd() {
        assert_eq!(median(&[-5, -3, -1, 0, 2]), Some(-1));
    }

    #[test]
    fn negative_even() {
        assert_eq!(median(&[-10, -5, 0, 5]), Some(-5));
    }

    #[test]
    fn all_same() {
        assert_eq!(median(&[5, 5, 5, 5, 5]), Some(5));
    }

    #[test]
    fn large_values() {
        assert_eq!(median(&[1000, 2000, 3000]), Some(2000));
    }

    #[test]
    fn mixed_positive_negative() {
        assert_eq!(median(&[-1, 0, 1]), Some(0));
    }

    #[test]
    fn even_length_large() {
        let v: Vec<i32> = (1..=100).collect();
        assert_eq!(median(&v), Some(50));
    }

    #[test]
    fn odd_length_large() {
        let v: Vec<i32> = (1..=99).collect();
        assert_eq!(median(&v), Some(50));
    }
}

fn main() {
    println!("median([1,2,3,4,5]) = {:?}", median(&[1, 2, 3, 4, 5]));
    println!("median([1,2,3,4]) = {:?}", median(&[1, 2, 3, 4]));
    println!("median([]) = {:?}", median(&[]));
    println!("\nRun: rustc --test --edition 2021 ex03_median.rs -o /tmp/t && /tmp/t");
}

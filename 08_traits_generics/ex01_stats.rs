// ex01_stats.rs
// Exercise: Generic stats — mean, median, mode on &[T]
//
// Implement generic functions to compute statistics on a slice of numbers.
// The functions should work with both integers and floats.
//
// Strategy: Convert to f64 via Into<f64> for mean. Median and mode are harder
// for f64 (no Ord), so we demonstrate two approaches:
//   - median works on types that implement Into<f64> + using PartialOrd
//   - mode uses a HashMap + Eq + Hash
//
// Run: rustc --edition 2021 ex01_stats.rs && ./ex01_stats

use std::collections::HashMap;
use std::hash::Hash;

/// Mean (average) of a slice as f64. Returns None for empty slices.
/// T only needs to be representable as f64.
fn mean<T>(data: &[T]) -> Option<f64>
where
    T: Copy + Into<f64>,
{
    if data.is_empty() {
        return None;
    }
    let sum: f64 = data.iter().map(|&x| x.into()).sum();
    let count = data.len() as f64;
    Some(sum / count)
}

/// Median of a slice. Works with types that implement Into<f64> + PartialOrd.
/// For even-length slices, returns the average of the two middle values.
/// Returns None for empty slices.
fn median<T>(data: &[T]) -> Option<f64>
where
    T: Copy + Into<f64> + PartialOrd,
{
    if data.is_empty() {
        return None;
    }
    let mut sorted: Vec<T> = data.to_vec();
    // Use sort_by with PartialOrd comparison since f64 doesn't implement Ord.
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let len = sorted.len();
    if len % 2 == 1 {
        // Odd length: middle element
        Some(sorted[len / 2].into())
    } else {
        // Even length: average of two middle elements
        let a: f64 = sorted[len / 2 - 1].into();
        let b: f64 = sorted[len / 2].into();
        Some((a + b) / 2.0)
    }
}

/// Mode (most frequent value). Returns a vec of the most frequent value(s).
/// T must implement Clone + Eq + Hash (for HashMap).
fn mode<T>(data: &[T]) -> Vec<T>
where
    T: Clone + Eq + Hash,
{
    if data.is_empty() {
        return vec![];
    }
    let mut freq: HashMap<&T, usize> = HashMap::new();
    for item in data {
        *freq.entry(item).or_insert(0) += 1;
    }
    let max_count = freq.values().cloned().max().unwrap_or(0);
    freq.into_iter()
        .filter(|(_, count)| *count == max_count)
        .map(|(key, _)| key.clone())
        .collect()
}

/// A helper struct for testing with a custom type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Score(u32);

impl From<Score> for f64 {
    fn from(s: Score) -> f64 {
        s.0 as f64
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

// Note: f64 implements PartialOrd but NOT Ord, so we use sort_by with partial_cmp.

fn main() {
    // ── Integer tests ────────────────────────────────────────────────────────
    let ints = vec![5, 3, 8, 1, 9, 2, 7, 4, 6];
    println!("Integers: {:?}", ints);
    println!("  mean:   {:.2}", mean(&ints).unwrap());
    println!("  median: {:.2}", median(&ints).unwrap());
    println!("  mode:   {:?}", mode(&ints));

    // ── Float tests (mean and median only — mode requires Eq+Hash which f64 lacks) ──
    let floats = vec![2.5, 1.5, 3.5, 2.5, 4.0];
    println!("Floats: {:?}", floats);
    println!("  mean:   {:.2}", mean(&floats).unwrap());
    println!("  median: {:.2}", median(&floats).unwrap());
    // Mode on f64 is tricky (no Eq/Hash due to NaN) — demonstrated with integers instead.

    // ── Mode with tie ─────────────────────────────────────────────────────────
    let ties = vec![1, 2, 2, 3, 3, 4];
    println!("Ties: {:?}", ties);
    println!("  mode: {:?}", mode(&ties)); // [2, 3]

    // ── Empty slice ──────────────────────────────────────────────────────────
    let empty: Vec<i32> = vec![];
    println!("\nEmpty slice:");
    println!("  mean:   {:?}", mean(&empty));
    println!("  median: {:?}", median(&empty));
    println!("  mode:   {:?}", mode(&empty));

    // ── Custom type ──────────────────────────────────────────────────────────
    let scores: Vec<Score> = vec![
        Score(85),
        Score(92),
        Score(78),
        Score(92),
        Score(85),
        Score(100),
    ];
    println!("\nScores: {:?}", scores);
    println!("  mean:   {:.2}", mean(&scores).unwrap());
    println!("  median: {:.2}", median(&scores).unwrap());
    println!("  mode:   {:?}", mode(&scores));

    // ── Assertions ───────────────────────────────────────────────────────────
    assert!((mean(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap() - 5.0).abs() < 1e-10);
    assert!((median(&vec![1, 3, 5, 7, 9]).unwrap() - 5.0).abs() < 1e-10);
    assert!((median(&vec![1, 2, 3, 4]).unwrap() - 2.5).abs() < 1e-10);
    assert_eq!(mode(&vec![1i32, 1, 2, 2, 3]), vec![1, 2]);
    assert_eq!(mode::<i32>(&empty), Vec::<i32>::new());
    assert!(mean::<i32>(&empty).is_none());
    assert!(median::<i32>(&empty).is_none());

    println!("\nAll assertions passed! ✓");
}

// ex03_sort_test.rs — Test sort functions: empty, single, sorted, reverse-sorted, duplicates
//
// Implement a simple sort function and test it against various inputs.
// Edge cases: empty vec, single element, already sorted, reverse sorted,
// containing duplicates, all same value.
//
// Run: rustc --edition 2021 --test ex03_sort_test.rs && ./ex03_sort_test

// --- Implement sort ---

/// Sorts a mutable slice of i32 using bubble sort (simple, not efficient).
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break; // already sorted
        }
    }
}

/// Checks if a slice is sorted in ascending order.
fn is_sorted(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    // --- bubble_sort ---

    #[test]
    fn test_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr));
        assert!(arr.is_empty());
    }

    #[test]
    fn test_sort_single() {
        let mut arr = vec![42];
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr));
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_sort_two_elements() {
        let mut arr = vec![2, 1];
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr));
        assert_eq!(arr, vec![1, 2]);
    }

    #[test]
    fn test_sort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr));
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr));
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_random() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr));
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_sort_duplicates() {
        let mut arr = vec![3, 1, 3, 1, 3];
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr));
        assert_eq!(arr, vec![1, 1, 3, 3, 3]);
    }

    #[test]
    fn test_sort_all_same() {
        let mut arr = vec![7, 7, 7, 7, 7];
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr));
        assert_eq!(arr, vec![7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_sort_negative_numbers() {
        let mut arr = vec![-5, 3, -1, 0, 2, -10];
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr));
        assert_eq!(arr, vec![-10, -5, -1, 0, 2, 3]);
    }

    #[test]
    fn test_sort_large_sorted() {
        let mut arr: Vec<i32> = (0..100).collect();
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn test_sort_large_reverse() {
        let mut arr: Vec<i32> = (0..100).rev().collect();
        bubble_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    // --- is_sorted (test the helper too) ---

    #[test]
    fn test_is_sorted_empty() {
        let arr: Vec<i32> = vec![];
        assert!(is_sorted(&arr));
    }

    #[test]
    fn test_is_sorted_single() {
        assert!(is_sorted(&[42]));
    }

    #[test]
    fn test_is_sorted_true() {
        assert!(is_sorted(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_is_sorted_false() {
        assert!(!is_sorted(&[1, 3, 2, 4]));
    }

    #[test]
    fn test_is_sorted_with_duplicates() {
        assert!(is_sorted(&[1, 2, 2, 3]));
    }
}

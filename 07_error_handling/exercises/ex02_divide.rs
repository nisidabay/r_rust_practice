// ex02_divide — Safe division returning Result, handle divide by zero
//
// Problem: Write a function `safe_divide(a: i32, b: i32) -> Result<i32, String>`
// that returns Ok(a / b) or an Err with a message if b is 0.
//
// Then write a function `process_numbers(nums: &[i32]) -> Result<i32, String>`
// that takes pairs from the slice and divides them sequentially using ?,
// returning the cumulative product. If the slice has an odd number of elements,
// ignore the last element.
//
// Example: process_numbers(&[10, 2, 20, 4]) => Ok(10/2 * 20/4) = Ok(5 * 5) = Ok(25)
// Example: process_numbers(&[10, 0, 20, 4]) => Err("division by zero")

fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn process_numbers(nums: &[i32]) -> Result<i32, String> {
    let mut product = 1i32;
    for chunk in nums.chunks(2) {
        if chunk.len() < 2 {
            break; // ignore the last element if odd count
        }
        let quotient = safe_divide(chunk[0], chunk[1])?;
        product *= quotient;
    }
    Ok(product)
}

fn main() {
    // Test safe_divide
    println!("safe_divide:");
    println!("  10 / 2 = {:?}", safe_divide(10, 2));
    println!("  10 / 0 = {:?}", safe_divide(10, 0));
    println!("  -8 / 3 = {:?}", safe_divide(-8, 3));

    println!();

    // Test process_numbers
    let tests: [&[i32]; 4] = [
        &[10, 2, 20, 4],
        &[10, 0, 20, 4],
        &[100, 5, 2, 2, 9, 3],
        &[1],
    ];

    for (i, test) in tests.iter().enumerate() {
        println!("process_numbers test {i}: {:?} -> {:?}", test, process_numbers(test));
    }
}

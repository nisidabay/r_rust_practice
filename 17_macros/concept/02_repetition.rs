// 02_repetition.rs — $()* $()+ repetition, separators
//
// Repetition is the "loop" of macro_rules!. The syntax:
//   $($pattern)SEPARATOR REPEAT_OP
// where SEPARATOR is optional, and REPEAT_OP is * (zero+) or + (one+).
//
// Inside the pattern, $x refers to the current repetition binding.

use std::fmt::Write;

/// Build a comma-separated list from repeated expressions.
macro_rules! csv {
    ($($x:expr),*) => {
        {
            let mut s = String::new();
            $(
                write!(&mut s, "{}, ", $x).unwrap();
            )*
            s
        }
    };
    ($($x:expr),+) => {
        csv!($($x),*)
    };
}

/// Sum all arguments. The separator is `+` (not comma), using `*` for zero-or-more.
macro_rules! sum_of {
    ($($x:expr),+ $(,)?) => {{
        let mut sum = 0;
        $(
            sum += $x;
        )+
        sum
    }};
}

/// Build a Vec of repeated items (simpler than std vec![]).
macro_rules! my_vec {
    // With trailing comma support
    ($($x:expr),+ $(,)?) => {{
        let mut v = Vec::new();
        $(
            v.push($x);
        )+
        v
    }};
    // Empty vec
    () => {
        Vec::<i32>::new()
    };
}

/// Demonstrate repetition with different separators.
macro_rules! joined_by {
    ($sep:expr; $($x:expr),+) => {{
        let mut s = String::new();
        let mut first = true;
        $(
            if !first {
                write!(&mut s, "{}", $sep).unwrap();
            }
            write!(&mut s, "{}", $x).unwrap();
            first = false;
        )+
        s
    }};
}

fn main() {
    println!("=== Repetition in Macros ===");

    // csv! — comma separated list builder
    let s1 = csv!(1, 2, 3, 4);
    println!("csv: {}", s1);

    // Empty csv
    let s2 = csv!();
    println!("csv (empty): '{}'", s2);

    // Sum macro
    let total = sum_of!(10, 20, 30, 40);
    println!("Sum: {}", total);

    // my_vec
    let v = my_vec!("a", "b", "c");
    println!("my_vec: {:?}", v);

    let empty: Vec<i32> = my_vec!();
    println!("my_vec (empty): {:?}", empty);

    // Trailing comma
    let v2 = my_vec!(1, 2, 3,);
    println!("my_vec (trailing comma): {:?}", v2);

    // Custom separator
    let s3 = joined_by!(" :: "; "alpha", "beta", "gamma");
    println!("Joined by ' :: ' : {}", s3);

    println!("\nKey syntax: $($pattern)SEP* (zero+) or $($pattern)SEP+ (one+)");
    println!("Inside: $x refers to each captured token in the repetition.");
}

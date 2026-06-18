// BONUS EXERCISE: Fix the lifetime errors in this code.
// The compiler will tell you exactly what's wrong — your job is to fix it.
// Read the error messages carefully and apply the right fix.
//
// This program demonstrates common lifetime mistakes:
// 1. Forgetting a lifetime annotation on a struct
// 2. Returning a reference to a local variable
// 3. Outliving borrowed data
// 4. Multiple reference return without matching lifetimes
//
// THERE ARE 5 COMPILE ERRORS. Fix all of them.

// BUG 1: Struct holding a reference needs a lifetime parameter
struct RefHolder {
    value: &str,
}

// BUG 2: This function tries to return a reference to its own local data
fn make_ref() -> &str {
    let local = String::from("I'm local");
    &local
}

// BUG 3: This struct has two references with potentially different lifetimes
struct TwoRefs {
    first: &str,
    second: &str,
}

impl TwoRefs {
    fn new(first: &str, second: &str) -> Self {
        TwoRefs { first, second }
    }

    // BUG 4: This function takes two unrelated references and tries to return one
    fn first(&self) -> &str {
        self.first
    }

    fn longest(x: &str, y: &str) -> &str {
        if x.len() >= y.len() { x } else { y }
    }
}

fn main() {
    // This should work — fix the struct definitions above
    let data = String::from("hello");
    let holder = RefHolder { value: &data };
    println!("Holder value: {}", holder.value);

    // This should NOT compile at all (can't return local ref)
    // But after fixing make_ref, comment this out — it's fundamentally wrong
    // let r = make_ref();
    // println!("Make ref: {}", r);

    // This should compile after fixing TwoRefs
    let s1 = String::from("first string");
    let s2 = String::from("second string is longer");
    let pair = TwoRefs::new(&s1, &s2);
    println!("First: {}", pair.first());

    // This should work after fixing longest
    let longest_str = TwoRefs::longest(&s1, &s2);
    println!("Longest: {}", longest_str);
}

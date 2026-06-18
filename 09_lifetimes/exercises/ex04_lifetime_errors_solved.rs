// ex04_lifetime_errors_solved.rs — Fixed version of the lifetime error exercise.
//
// This is the corrected version of ex04_lifetime_errors.rs.
// See the comments for what was fixed.

// FIX 1: Added lifetime parameter 'a to the struct
struct RefHolder<'a> {
    value: &'a str,
}

// FIX 2: Can't return a reference to a local — return an owned String instead.
fn make_ref() -> String {
    String::from("I'm local")
}

// FIX 3: Added lifetime parameters for both references
struct TwoRefs<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

impl<'a, 'b> TwoRefs<'a, 'b> {
    fn new(first: &'a str, second: &'b str) -> Self {
        TwoRefs { first, second }
    }

    fn first(&self) -> &'a str {
        self.first
    }

    // FIX 4: Added lifetime annotation linking both parameters to the output
    fn longest<'c>(x: &'c str, y: &'c str) -> &'c str {
        if x.len() >= y.len() { x } else { y }
    }
}

fn main() {
    let data = String::from("hello");
    let holder = RefHolder { value: &data };
    println!("Holder value: {}", holder.value);

    // Now returns an owned String instead of a dangling reference
    let r = make_ref();
    println!("Make ref: {r}");

    let s1 = String::from("first string");
    let s2 = String::from("second string is longer");
    let pair = TwoRefs::new(&s1, &s2);
    println!("First: {}", pair.first());

    let longest_str = TwoRefs::longest(&s1, &s2);
    println!("Longest: {longest_str}");
}

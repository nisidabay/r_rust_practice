// 04_multiple_lifetime.rs
// Multiple lifetime parameters — when 'a and 'b are different.
//
// Sometimes a function has multiple references that live for DIFFERENT durations.
// You've seen 'a: 'b syntax (lifetime subtyping), but actually using multiple
// lifetime annotations explicitly is straightforward: just name them differently.
//
// Each lifetime parameter tracks ONE set of references that share a lifetime.
//
// Run: rustc --edition 2021 04_multiple_lifetime.rs && ./04_multiple_lifetime

/// Returns the longer of two strings — but only if x is non-empty.
/// x has lifetime 'a, y has lifetime 'b (they CAN differ).
/// The return type is 'a because we always return x or a substring of x.
fn first_or_longer<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.is_empty() {
        // The empty string is valid for any lifetime — it's a static property
        ""
    } else if x.len() >= y.len() {
        x
    } else {
        // Hmm, we want to return y but it has lifetime 'b, not 'a!
        // This would fail unless 'a is at least as long as 'b.
        // We can only return x... so let's just return x.
        // (This restriction is the point of the exercise.)
        x
    }
}

/// Returns a reference to a substring within 'source', delimited by markers
/// that come from a different string. The markers don't appear in the output,
/// so they can have a different/shorter lifetime.
fn extract_between<'a, 'b>(source: &'a str, start_marker: &'b str, end_marker: &'b str) -> Option<&'a str> {
    let start = source.find(start_marker)?;
    let start = start + start_marker.len();
    let end = source[start..].find(end_marker)?;
    Some(&source[start..start + end])
}

/// A struct with TWO different lifetimes.
/// Each field can have its own lifetime constraint.
#[derive(Debug)]
struct TwoSources<'a, 'b> {
    primary: &'a str,
    secondary: &'b str,
}

impl<'a, 'b> TwoSources<'a, 'b> {
    /// Return the longer of the two references — but we can only promise
    /// at MOST the shorter lifetime. Here we just return &str with the
    /// struct's combined scope.
    fn longer(&self) -> &str {
        if self.primary.len() >= self.secondary.len() {
            self.primary
        } else {
            self.secondary
        }
    }
}

/// In some cases, we need to ensure one lifetime outlives another.
/// The syntax 'a: 'b means "'a outlives 'b" (is at least as long).
/// This lets us return a reference with the SHORTER lifetime.
fn choose_first<'a, 'b: 'a>(x: &'a str, y: &'b str, flag: bool) -> &'a str {
    // We promise: the returned reference lives at least 'a.
    // If flag is true, we return y which has lifetime 'b.
    // For this to work, 'b must outlive 'a (i.e., 'b: 'a).
    // Syntax note: in the function signature we write `'b: 'a` meaning "'b outlives 'a"
    if flag { y } else { x }
}

fn main() {
    // --- first_or_longer with different lifetimes ---
    let owned = String::from("Hello from Rust!");
    {
        let temp = String::from("temp string");
        // owned lives longer than temp
        let result = first_or_longer(&owned, &temp);
        println!("first_or_longer: '{result}'");
        // result borrows from owned ('a), not temp ('b) — it's fine ✓
    }

    // --- extract_between ---
    let data = "The quick brown fox jumps over the lazy dog";
    let start = "brown ";
    let end = " jumps";
    match extract_between(data, start, end) {
        Some(extracted) => println!("extract_between: '{extracted}'"),
        None => println!("no match"),
    }

    // Markers can be short-lived
    {
        let start2 = "The ";
        let end2 = " lazy";
        match extract_between(data, start2, end2) {
            Some(extracted) => println!("extract_between with temp markers: '{extracted}'"),
            None => println!("no match"),
        }
    }
    // Markers dropped, data still alive — output references borrow from data ✓

    // --- TwoSources with distinct lifetimes ---
    let long_lived = String::from("This is primary data that lives a long time");
    let short_lived = String::from("temp secondary");
    let sources = TwoSources {
        primary: &long_lived,
        secondary: &short_lived,
    };
    println!("TwoSources: primary='{}', secondary='{}'", sources.primary, sources.secondary);
    println!("longer: '{}'", sources.longer());

    // --- Lifetime subtyping: 'a: 'b in action ---
    {
        let short = String::from("short lived");
        // 'a = long_lived (outer scope), 'b = short (inner scope)
        // Because 'b: 'a, the function can return &'a — but since short is
        // actually shorter, we can only get &long_lived here.
        // The flag controls which one we return conceptually.
        let chosen = choose_first(&long_lived, &short, true);
        println!("choose_first(short flag): '{chosen}'");
        // This compiles because 'b: 'a lets us return y (&'b) as &'a
        // BUT in practice, short lives shorter, so this only works because
        // 'a actually outlives 'b... let's re-examine.
        // 
        // Actually for `choose_first(&long_lived, &short, true)`:
        //   &long_lived: &'a where 'a = lifetime of long_lived
        //   &short: &'b where 'b = lifetime of short
        //   We said 'b: 'a, but 'b is SHORTER than 'a — this can't work!
        // 
        // Let me fix the call: we need 'a to be the SHORTER lifetime.
        // The constraint 'b: 'a means 'b >= 'a — 'b lives longer than or equal to 'a.
    }

    // Fixed: proper lifetimes for choose_first
    let outer = String::from("outer data");
    let chosen_result;
    {
        let inner = String::from("inner data");
        // 'a = inner (shorter), 'b = outer (longer)
        // 'b: 'a holds because outer > inner
        let result = choose_first(&inner, &outer, true);
        // result is &'a str (inner lifetime) — can't be used past inner's scope
        chosen_result = result;
        println!("choose_first returns: '{chosen_result}'");
    }
    // println!("chosen_result: '{chosen_result}'"); // ERROR: borrowed value dropped

    println!("\n✓ Multiple lifetimes work correctly!");
}

// 05_multiple.rs — Multiple lifetime parameters <'a, 'b>
//
// When a function has multiple references with DIFFERENT lifetimes,
// you need multiple lifetime parameters. The output can be tied to
// one specific input.

// Two different lifetimes: 'a for the first string, 'b for the second.
// The output borrows from x (lifetime 'a), not y.
fn choose_first<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x  // y's lifetime 'b doesn't constrain the output
}

// Lifetime constraints: tell the compiler that 'b must outlive 'a
// Syntax: 'b: 'a means "'b lives at least as long as 'a"
fn choose_first_with_constraint<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    // Without the constraint, Rust can't prove y lives long enough for x's scope.
    // With 'b: 'a, we assert y outlives x.
    if x.len() > y.len() { x } else { y }
}

// Struct with two different lifetime references
struct Messenger<'a, 'b> {
    sender: &'a str,
    receiver: &'b str,
}

impl<'a, 'b> Messenger<'a, 'b> {
    fn new(sender: &'a str, receiver: &'b str) -> Self {
        Messenger { sender, receiver }
    }

    fn describe(&self) -> String {
        format!("From '{}' to '{}'", self.sender, self.receiver)
    }
}

fn main() {
    let s1 = String::from("long-lived data");

    let result;
    {
        let s2 = String::from("temporary");
        // choose_first only ties output to s1 ('a), so s2 can die
        result = choose_first(&s1, &s2);
        println!("Chose first (s2 can die): '{}'", result);
    } // s2 dropped — result still valid because it points to s1

    // With constraint: output tied to both
    let a = String::from("Alice");
    let b = String::from("Bob");
    let chosen = choose_first_with_constraint(&a, &b);
    println!("Chose with constraint: '{}'", chosen);

    // Messenger with two lifetimes
    let sender = String::from("alice@example.com");
    let receiver = String::from("bob@example.com");
    let msg = Messenger::new(&sender, &receiver);
    println!("Message: {}", msg.describe());

    // Different lifetimes in action: sender outlives receiver
    let msg2;
    let temp = String::from("temp_user");
    {
        let permanent = String::from("permanent_user");
        msg2 = Messenger::new(&permanent, &temp);
        println!("Messenger with distinct lifetimes: {}", msg2.describe());
    }

    println!("Multiple lifetimes: <'a, 'b> lets you connect different scopes.");
}

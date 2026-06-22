// ex04_bonus_messenger.rs — Struct with TWO different lifetime parameters
//
// Create a struct Message<'a, 'b> that holds two references with different
// lifetimes. Demonstrate that they can differ — one can outlive the other.

// TODO: Complete the struct with two lifetime params and implement:
//   - new(sender, body) constructor
//   - sender() -> &'a str
//   - body() -> &'b str
//   - full_message() -> String that concatenates both

struct Message<'a, 'b> {
    sender: &'a str,
    body: &'b str,
}

impl<'a, 'b> Message<'a, 'b> {
    fn new(sender: &'a str, body: &'b str) -> Self {
        Message { sender, body }
    }

    fn sender(&self) -> &'a str {
        self.sender
    }

    fn body(&self) -> &'b str {
        self.body
    }

    fn full_message(&self) -> String {
        format!("{}: {}", self.sender, self.body)
    }
}

fn main() {
    // The sender lives for the entire function
    let sender = String::from("system@example.com");

    let full_msg;
    {
        // The body is temporary — shorter lifetime
        let body = String::from("Server rebooting at midnight");

        // Message borrows both, but they have different lifetimes
        let msg = Message::new(&sender, &body);
        println!("Sender: '{}'", msg.sender());
        println!("Body: '{}'", msg.body());
        println!("Full: {}", msg.full_message());

        // full_message returns an owned String, so it doesn't borrow
        full_msg = msg.full_message();
    } // body is dropped here — but full_msg is owned, so it survives

    println!("Full message survives the body: '{}'", full_msg);

    // Demonstrate that sender has a different (longer) lifetime
    let temp_sender = String::from("temp@example.com");
    let permanent_body = String::from("I outlive the sender!");
    let msg2 = Message::new(&temp_sender, &permanent_body);
    println!("Sender temporary, body permanent: {}", msg2.full_message());

    println!("\n<'a, 'b>: two lifetimes, two scopes, one struct.");
}

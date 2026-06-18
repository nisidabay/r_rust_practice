// Enum variants can carry data — each variant can have different types and amounts of data.
// This is Rust's way of doing "sum types" (like tagged unions).

fn main() {
    // --- Enums with different data shapes ---
    enum Message {
        Quit,                       // No data
        Move { x: i32, y: i32 },   // Named fields (like a struct)
        Write(String),              // Tuple variant with one String
        ChangeColor(i32, i32, i32), // Tuple variant with three i32s
    }

    // Create instances — each variant has its own shape.
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    // --- Matching on data enums ---
    fn process_message(msg: Message) {
        match msg {
            Message::Quit => println!("Quitting"),
            Message::Move { x, y } => println!("Move to ({x}, {y})"),
            Message::Write(text) => println!("Message: {text}"),
            Message::ChangeColor(r, g, b) => println!("Color: rgb({r}, {g}, {b})"),
        }
    }

    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    // Wait — msg3 was MOVED into process_message, but msg4 is still valid.
    process_message(msg4);

    // --- Methods on data enums ---
    impl Message {
        fn call(&self) {
            match self {
                Message::Quit => println!("Quit"),
                Message::Move { x, y } => println!("Move to ({x}, {y})"),
                Message::Write(text) => println!("Write: {text}"),
                Message::ChangeColor(r, g, b) => println!("Color: rgb({r}, {g}, {b})"),
            }
        }
    }

    let m = Message::Write(String::from("method call"));
    m.call();  // note: &self, so ownership is NOT moved

    // --- Enum for IP addresses ---
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Match with data binding
    match home {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {a}.{b}.{c}.{d}"),
        IpAddr::V6(addr) => println!("IPv6: {addr}"),
    }

    // match on loopback too
    match loopback {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {a}.{b}.{c}.{d}"),
        IpAddr::V6(addr) => println!("IPv6: {addr}"),
    }
}

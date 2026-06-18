// `if let` and `while let` are syntactic sugar for matching a single pattern.
// They're shorter than `match` when you only care about one variant.

fn main() {
    // --- if let basics ---
    // Long form with match:
    let some_value: Option<i32> = Some(42);
    match some_value {
        Some(x) => println!("match: The value is {x}"),
        _ => (),  // boilerplate
    }

    // Same thing with if let (much shorter):
    if let Some(x) = some_value {
        println!("if let: The value is {x}");
    }
    // No `else` needed — but you can add one.
    // The `else` branch runs when the pattern doesn't match.

    // --- if let with else ---
    let maybe_number: Option<i32> = None;

    if let Some(val) = maybe_number {
        println!("Got: {val}");
    } else {
        println!("Got nothing");
    }

    // --- if let with enum variants ---
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    let my_coin = Coin::Dime;

    // Only care about Quarters? Use if let.
    if let Coin::Quarter = my_coin {
        println!("It's a quarter!");
    } else {
        println!("Not a quarter");
    }

    // --- if let with data enums ---
    enum Message {
        Quit,
        Write(String),
        Move { x: i32, y: i32 },
    }

    let msg = Message::Write(String::from("hello"));

    if let Message::Write(text) = msg {
        println!("Message says: {text}");
    } else {
        println!("Not a write message");
    }
    // Note: msg is PARTIALLY MOVED if we destructure, but here it's borrowed because
    // we used a reference... actually let's be careful.
    // Actually for types that don't implement Copy, if let moves. Let's use a fresh one.

    let msg2 = Message::Move { x: 10, y: 20 };
    if let Message::Move { x, y } = msg2 {
        println!("Moved to ({x}, {y})");
    }

    // --- while let — loop while pattern matches ---
    let mut stack = vec![1, 2, 3, 4, 5];

    // Pop values until the stack is empty.
    // pop() returns Option<T> — Some if there's a value, None when empty.
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!();  // prints: 5 4 3 2 1

    // --- while let with mutable state ---
    let mut optional: Option<i32> = Some(0);

    // Keep incrementing while we have a value.
    while let Some(i) = optional {
        if i > 9 {
            println!("Reached {i}, stopping");
            optional = None;
        } else {
            println!("i = {i}");
            optional = Some(i + 1);
        }
    }
}

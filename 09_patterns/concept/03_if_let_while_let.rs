fn main() {
    // if let — match ONE variant concisely. Use when you only care about one case.
    // while let — loop as long as a pattern matches (common with Option).

    // --- if let basics ---
    let val: Option<i32> = Some(42);

    // Full match (verbose when you only want one case):
    match val {
        Some(v) => println!("The answer is {}", v),
        None => {} // boilerplate
    }

    // Same with if let (concise):
    if let Some(v) = val {
        println!("The answer is {}", v);
    }

    // --- if let with else ---
    let none_val: Option<i32> = None;
    if let Some(v) = none_val {
        println!("Found: {}", v);
    } else {
        println!("None — nothing to see");
    }

    // --- if let with struct destructuring ---
    #[derive(Debug)]
    enum Message {
        Quit,
        Write(String),
    }

    let msg = Message::Write(String::from("hello"));
    if let Message::Write(text) = &msg {
        println!("Wrote: {}", text);
    }

    // --- while let — loop while a pattern matches ---
    let mut stack = vec![1, 2, 3, 4, 5];

    println!("Popping:");
    while let Some(top) = stack.pop() {
        // This loop runs until pop() returns None
        print!("{} ", top);
    }
    println!();

    // --- Practical: parsing command-line args with while let ---
    let args = vec!["--name=Alice".to_string(), "--verbose".to_string()];
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        if let Some((key, val)) = arg.split_once('=') {
            println!("Arg: {} = {}", key, val);
        } else {
            println!("Flag: {}", arg);
        }
    }
}

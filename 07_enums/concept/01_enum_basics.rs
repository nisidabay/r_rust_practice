fn main() {
    // Compass directions as an enum — each variant IS a value, not a number
    // Rust enums are NOT integer constants (like C). They're distinct types.
    enum Direction {
        North,
        South,
        East,
        West,
    }

    // Pick a direction. This is a value of type Direction, not an int.
    let heading = Direction::North;

    // match is exhaustive — the compiler checks every variant is handled
    // Without 'East' and 'West', this would not compile.
    match heading {
        Direction::North => println!("↑ Going north"),
        Direction::South => println!("↓ Going south"),
        Direction::East  => println!("→ Going east"),
        Direction::West  => println!("← Going west"),
    }

    // -- Second example: a simple enum for HTTP methods --
    enum HttpMethod {
        Get,
        Post,
        Put,
        Delete,
    }

    let method = HttpMethod::Post;

    // match returns a value — each arm is an expression
    let desc = match method {
        HttpMethod::Get    => "fetch a resource",
        HttpMethod::Post   => "create a resource",
        HttpMethod::Put    => "update a resource",
        HttpMethod::Delete => "remove a resource",
    };
    println!("POST: {}", desc);
}

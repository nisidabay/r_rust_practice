// Enums let you define a type by enumerating its possible variants.
// Each variant is a distinct value — like a type-safe set of choices.

fn main() {
    // --- Basic enum definition ---
    // Each variant is a namespace under the enum name.
    enum Direction {
        North,
        South,
        East,
        West,
    }

    // Create enum values using :: syntax.
    let heading = Direction::North;
    let _south = Direction::South;
    let _east = Direction::East;
    let _west = Direction::West;

    // --- Matching on enums ---
    // Match is exhaustive — must handle every variant.
    fn describe_direction(dir: Direction) -> &'static str {
        match dir {
            Direction::North => "going up",
            Direction::South => "going down",
            Direction::East => "going right",
            Direction::West => "going left",
        }
    }
    println!("North: {}", describe_direction(heading));

    // --- Using enums in functions ---
    enum HttpStatus {
        Ok,
        NotFound,
        InternalServerError,
    }

    fn status_message(status: HttpStatus) -> &'static str {
        match status {
            HttpStatus::Ok => "200 OK",
            HttpStatus::NotFound => "404 Not Found",
            HttpStatus::InternalServerError => "500 Internal Server Error",
        }
    }

    println!("{}", status_message(HttpStatus::Ok));
    println!("{}", status_message(HttpStatus::NotFound));

    // --- Enums with impl blocks ---
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    impl TrafficLight {
        fn description(&self) -> &'static str {
            match self {
                TrafficLight::Red => "Stop",
                TrafficLight::Yellow => "Caution",
                TrafficLight::Green => "Go",
            }
        }
    }

    let light = TrafficLight::Red;
    println!("Red light: {}", light.description());

    // --- The _ wildcard in match ---
    // Use `_` or `other` for remaining variants when you don't need all.
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            _ => 25,  // Quarter and any future variants
        }
    }
    println!("Dime value: {}¢", value_in_cents(Coin::Dime));
}

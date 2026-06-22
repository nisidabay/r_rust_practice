// 02_trait_default.rs — Default implementations in traits
//
// Traits can provide default method bodies. Types can override them
// or use the default.

trait Greeter {
    // Method with a default implementation
    fn greet(&self) -> String {
        format!("Hello!")
    }

    // Method without default — must be implemented
    fn name(&self) -> String;
}

// --- Use the default ---
struct Robot;

impl Greeter for Robot {
    fn name(&self) -> String {
        "RobotX9000".to_string()
    }
    // greet() uses the default
}

// --- Override the default ---
struct FriendlyRobot;

impl Greeter for FriendlyRobot {
    fn name(&self) -> String {
        "Buddy".to_string()
    }

    fn greet(&self) -> String {
        format!("Hi, I'm {}! Nice to meet you!", self.name())
    }
}

// --- Default methods can call other trait methods ---
trait HasArea {
    fn area(&self) -> f64;

    // Default method that uses area()
    fn is_larger_than(&self, other: &Self) -> bool {
        self.area() > other.area()
    }
}

struct Circle {
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    // is_larger_than uses the default
}

fn main() {
    let robot = Robot;
    let buddy = FriendlyRobot;

    println!("Robot:      {} says \"{}\"", robot.name(), robot.greet());
    println!("Friendly:   {} says \"{}\"", buddy.name(), buddy.greet());

    let small = Circle { radius: 1.0 };
    let big = Circle { radius: 5.0 };
    println!("big > small? {}", big.is_larger_than(&small));
}

// 04_trait_default.rs
// Default method implementations in traits.
//
// Traits can provide default method bodies. Implementors only need to override
// the methods they want to customise; the rest come from the default.
// This enables the "template method" pattern and minimises boilerplate.
//
// Run: rustc --edition 2021 04_trait_default.rs && ./04_trait_default

use std::fmt::Display;

// ── Trait with a default method ──────────────────────────────────────────────

trait Greeter {
    /// The implementor MUST provide a name.
    fn name(&self) -> &str;

    /// Default implementation — works for any type, but can be overridden.
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}

// ── Implement with default greet ─────────────────────────────────────────────

struct Person {
    full_name: String,
}

impl Greeter for Person {
    fn name(&self) -> &str {
        &self.full_name
    }
    // greet() uses the default.
}

// ── Override the default ─────────────────────────────────────────────────────

enum Mood {
    Cheerful,
    Grumpy,
}

struct Robot {
    id: u32,
    mood: Mood,
}

impl Greeter for Robot {
    fn name(&self) -> &str {
        "Robot" // Robots don't have names by default.
    }

    // Override greet() completely.
    fn greet(&self) -> String {
        match self.mood {
            Mood::Cheerful => format!("Beep boop! I am {} #{}", self.name(), self.id),
            Mood::Grumpy => format!("Error: unwilling to greet. (User #{} is busy.)", self.id),
        }
    }
}

// ── Trait with multiple defaults calling each other ──────────────────────────

/// A trait modelling a basic shape that knows its own area and description.
/// Some methods have defaults that call other trait methods — the "template
/// method" pattern.
trait Shape {
    /// Every shape must report its area.
    fn area(&self) -> f64;

    /// A default that uses area() — implementors get this for free.
    fn description(&self) -> String {
        format!("A shape with area {:.2}", self.area())
    }

    /// A more detailed default that also calls description().
    fn show(&self) {
        println!("  => {}", self.description());
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    // description() and show() use defaults.
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // Override description for a custom message.
    fn description(&self) -> String {
        format!(
            "Rectangle {}×{} — area {:.2}",
            self.width,
            self.height,
            self.area()
        )
    }
    // show() still uses the default.
}

// ── Trait with default that requires Display ─────────────────────────────────

/// Marker trait that indicates a type can "shout" its representation.
trait Shout: Display {
    fn shout(&self) -> String {
        format!("{}!!!", self)
    }
}

/// A newtype wrapper so we can implement Shout (orphan rule: local trait + local type).
#[derive(Debug, Clone, Copy)]
struct LoudInt(i32);

impl Display for LoudInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Shout for LoudInt {} // All defaults.

fn main() {
    let alice = Person {
        full_name: "Alice".into(),
    };
    println!("Person: {}", alice.greet());

    let happy_robot = Robot {
        id: 42,
        mood: Mood::Cheerful,
    };
    println!("Robot cheer: {}", happy_robot.greet());

    let grumpy_robot = Robot {
        id: 99,
        mood: Mood::Grumpy,
    };
    println!("Robot grump: {}", grumpy_robot.greet());

    // Shapes with defaults
    let c = Circle { radius: 5.0 };
    println!("Circle:");
    c.show();

    let r = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    println!("Rectangle:");
    r.show();

    // Shout
    let n = LoudInt(42);
    println!("shout: {}", n.shout());
}

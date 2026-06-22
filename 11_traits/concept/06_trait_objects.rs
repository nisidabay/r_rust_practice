// 06_trait_objects.rs — &dyn Trait and Box<dyn Trait> for dynamic dispatch
//
// Trait objects enable polymorphism at runtime (like vtables in C++).
// Two forms: &dyn Trait (borrowed) and Box<dyn Trait> (owned).

trait Animal {
    fn make_sound(&self) -> String;
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        "Woof!".to_string()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        "Meow!".to_string()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

struct Cow {
    name: String,
}

impl Animal for Cow {
    fn make_sound(&self) -> String {
        "Moo!".to_string()
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

// --- &dyn Trait as parameter (borrowed) ---
fn print_animal(animal: &dyn Animal) {
    println!("{} says {}", animal.name(), animal.make_sound());
}

// --- Box<dyn Trait> as return type ---
// Can return different types at runtime
fn create_animal(kind: &str) -> Box<dyn Animal> {
    match kind {
        "dog" => Box::new(Dog { name: "Rex".to_string() }),
        "cat" => Box::new(Cat { name: "Whiskers".to_string() }),
        "cow" => Box::new(Cow { name: "Bessie".to_string() }),
        _ => Box::new(Dog { name: "Unknown".to_string() }),
    }
}

// --- Vec<Box<dyn Trait>> — heterogeneous collection ---
fn main() {
    // &dyn Trait — borrow
    let dog = Dog { name: "Rex".to_string() };
    let cat = Cat { name: "Whiskers".to_string() };
    let cow = Cow { name: "Bessie".to_string() };

    print_animal(&dog);
    print_animal(&cat);
    print_animal(&cow);

    // Box<dyn Trait> — owned
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: "Rex".to_string() }),
        Box::new(Cat { name: "Whiskers".to_string() }),
        Box::new(Cow { name: "Bessie".to_string() }),
        create_animal("dog"),
        create_animal("cat"),
    ];

    println!("\n--- All animals ---");
    for animal in &animals {
        println!("{} says {}", animal.name(), animal.make_sound());
    }

    // --- Static dispatch vs dynamic dispatch ---
    println!("\n--- Key difference ---");
    println!("Static dispatch  (generics):   one version per type, no overhead");
    println!("Dynamic dispatch (&dyn Trait):  vtable lookup, runtime cost");
    println!("Use generics when possible. Use dyn Trait when types are unknown at compile time.");

    // --- Object safety ---
    println!("\n--- Object safety rules ---");
    println!("A trait can be used as a trait object ONLY if:");
    println!("  1. Self is not in method signatures (other than &self, &mut self, Box<Self>)");
    println!("  2. Methods don't return Self");
    println!("  3. Methods don't have generic type parameters");
}

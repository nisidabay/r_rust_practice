fn main() {
    // Enums can have methods (via impl blocks), just like structs
    // This lets enums carry BOTH data AND behavior

    #[derive(Debug)]
    enum Temperature {
        Celsius(f64),
        Fahrenheit(f64),
        Kelvin(f64), // Bonus variant to show associated constants
    }

    impl Temperature {
        // Associated constant — belongs to the type, not an instance
        // Like a static const in C, but namespaced under the enum
        const ABSOLUTE_ZERO_C: f64 = -273.15;

        // Method — takes &self and operates on an instance
        fn to_celsius(&self) -> f64 {
            match self {
                Temperature::Celsius(c) => *c,
                Temperature::Fahrenheit(f) => (f - 32.0) * 5.0 / 9.0,
                Temperature::Kelvin(k) => k - 273.15,
            }
        }

        // Another method
        fn description(&self) -> String {
            let c = self.to_celsius();
            let category = if c < 0.0 {
                "freezing"
            } else if c < 20.0 {
                "cold"
            } else if c < 30.0 {
                "warm"
            } else {
                "hot"
            };
            format!("{} — {} ({:.1}°C)", category, c, c)
        }

        // Associated function (no &self) — like a constructor
        // Called as Temperature::from_celsius(100.0)
        fn from_celsius(c: f64) -> Self {
            Temperature::Celsius(c)
        }

        // Check if temperature is below absolute zero (impossible)
        fn is_valid(&self) -> bool {
            self.to_celsius() >= Self::ABSOLUTE_ZERO_C
        }
    }

    // Using the enum and its methods
    let temps = vec![
        Temperature::Celsius(25.0),
        Temperature::Fahrenheit(98.6),
        Temperature::Kelvin(0.0),
        Temperature::from_celsius(-300.0), // below absolute zero!
    ];

    for t in &temps {
        println!("{:?}:", t);
        println!("  -> {}°C", t.to_celsius());
        println!("  -> {}", t.description());
        println!("  -> valid: {}", t.is_valid());
    }
}

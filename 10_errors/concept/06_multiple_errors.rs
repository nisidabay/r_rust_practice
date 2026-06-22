fn main() {
    // Multiple error types — combine io::Error, ParseIntError, and custom errors
    // Two strategies: Box<dyn Error> or a custom enum wrapping each error type

    use std::fmt;
    use std::fs;
    use std::num::ParseIntError;

    // --- Strategy 1: Box<dyn Error> — simple, loses specific error info ---
    fn read_number_simple(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;     // io::Error -> Box<dyn Error>
        let num = contents.trim().parse::<i32>()?;    // ParseIntError -> Box<dyn Error>
        Ok(num)
    }

    match read_number_simple("/etc/hostname") {
        Ok(n) => println!("Number: {}", n),
        Err(e) => println!("Simple error: {}", e),
    }

    // --- Strategy 2: Custom enum wrapping each error type (better) ---
    #[derive(Debug)]
    enum ConfigError {
        Io(std::io::Error),
        Parse(ParseIntError),
        EmptyFile,
    }

    // Implement Display
    impl fmt::Display for ConfigError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ConfigError::Io(e) => write!(f, "I/O error: {}", e),
                ConfigError::Parse(e) => write!(f, "Parse error: {}", e),
                ConfigError::EmptyFile => write!(f, "File is empty"),
            }
        }
    }

    impl std::error::Error for ConfigError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                // Provide cause for wrapped errors (useful for error chains)
                ConfigError::Io(e) => Some(e),
                ConfigError::Parse(e) => Some(e),
                ConfigError::EmptyFile => None,
            }
        }
    }

    // Implement From traits so ? works automatically!
    impl From<std::io::Error> for ConfigError {
        fn from(e: std::io::Error) -> Self {
            ConfigError::Io(e)
        }
    }

    impl From<ParseIntError> for ConfigError {
        fn from(e: ParseIntError) -> Self {
            ConfigError::Parse(e)
        }
    }

    // Now ? works seamlessly with different error types
    fn read_number_better(path: &str) -> Result<i32, ConfigError> {
        let contents = fs::read_to_string(path)?;  // io::Error -> ConfigError via From
        if contents.trim().is_empty() {
            return Err(ConfigError::EmptyFile);
        }
        let num = contents.trim().parse::<i32>()?; // ParseIntError -> ConfigError via From
        Ok(num)
    }

    match read_number_better("/etc/hostname") {
        Ok(n) => println!("Number: {}", n),
        Err(e) => println!("Better error: {}", e),
    }

    println!("\nBox<dyn Error> = simple but lose type info");
    println!("Custom enum    = more work but preserve error details");
}

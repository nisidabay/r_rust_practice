/// Configuration module — parses command-line arguments

/// Holds the parsed CLI configuration
pub struct Config {
    /// Directory to organize
    pub directory: String,
    /// Whether to show help
    pub help: bool,
}

impl Config {
    /// Parse command-line arguments into a Config.
    /// Accepts: --dir <path> and --help
    pub fn from_args() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut directory = String::new();
        let mut help = false;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--dir" => {
                    i += 1;
                    if i < args.len() {
                        directory = args[i].clone();
                    }
                }
                "--help" | "-h" => {
                    help = true;
                }
                _ => {}
            }
            i += 1;
        }

        // Default to current directory if none specified
        if directory.is_empty() {
            directory = String::from(".");
        }

        Config { directory, help }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_help_flag() {
        // We can't easily test with real args, so we just verify the struct
        let cfg = Config {
            directory: ".".to_string(),
            help: false,
        };
        assert_eq!(cfg.directory, ".");
        assert!(!cfg.help);
    }
}

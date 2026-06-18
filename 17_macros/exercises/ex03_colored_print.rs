// ex03_colored_print.rs — Build a macro for colored terminal output (ANSI codes)
//
// Exercise: Create macros that add ANSI color escapes to terminal output.
// Using macro_rules! to wrap text in ANSI escape sequences.

/// Color codes for terminal output.
mod color {
    pub const RESET: &str = "\x1b[0m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
}

/// Print text in a color.
macro_rules! cprint {
    ($color:expr, $($arg:tt)*) => {
        print!("{}{}{}", $color, format_args!($($arg)*), color::RESET);
    };
}

/// Print line in a color.
macro_rules! cprintln {
    ($color:expr, $($arg:tt)*) => {
        println!("{}{}{}", $color, format_args!($($arg)*), color::RESET);
    };
}

/// Convenience macros for specific colors.
macro_rules! red {
    ($($arg:tt)*) => { cprintln!(color::RED, $($arg)*); };
}
macro_rules! green {
    ($($arg:tt)*) => { cprintln!(color::GREEN, $($arg)*); };
}
macro_rules! yellow {
    ($($arg:tt)*) => { cprintln!(color::YELLOW, $($arg)*); };
}
macro_rules! blue {
    ($($arg:tt)*) => { cprintln!(color::BLUE, $($arg)*); };
}
macro_rules! cyan {
    ($($arg:tt)*) => { cprintln!(color::CYAN, $($arg)*); };
}
macro_rules! magenta {
    ($($arg:tt)*) => { cprintln!(color::MAGENTA, $($arg)*); };
}

/// Bold text.
macro_rules! bold {
    ($($arg:tt)*) => { cprintln!(color::BOLD, $($arg)*); };
}

/// Dim text.
macro_rules! dim {
    ($($arg:tt)*) => { cprintln!(color::DIM, $($arg)*); };
}

/// Styled: color + style.
macro_rules! styled {
    ($color:expr, $style:expr, $($arg:tt)*) => {
        println!("{}{}{}{}",
                 $color, $style, format_args!($($arg)*), color::RESET);
    };
}

/// A simple status display with colored prefix.
macro_rules! status {
    (ok $($arg:tt)*) => {
        cprint!(color::GREEN, "✓ ");
        println!($($arg)*);
    };
    (warn $($arg:tt)*) => {
        cprint!(color::YELLOW, "⚠ ");
        println!($($arg)*);
    };
    (err $($arg:tt)*) => {
        cprint!(color::RED, "✗ ");
        println!($($arg)*);
    };
    (info $($arg:tt)*) => {
        cprint!(color::BLUE, "ℹ ");
        println!($($arg)*);
    };
}

/// Rainbow text: alternates colors across characters.
/// This one is more complex, using recursion.
macro_rules! rainbow {
    () => {};
    ($ch:tt) => {
        cprint!(color::RED, "{}", stringify!($ch));
    };
}

fn main() {
    println!("=== Colored Print Macros ===\n");

    // Basic colored output
    red!("This is red");
    green!("This is green");
    yellow!("This is yellow");
    blue!("This is blue");
    cyan!("This is cyan");
    magenta!("This is magenta");

    // Style
    bold!("This is bold");
    dim!("This is dim");

    // Combined style
    styled!(color::GREEN, color::BOLD, "Bold green text!");
    styled!(color::RED, color::DIM, "Dim red text!");

    // Status macros
    status!(ok "Operation completed successfully");
    status!(warn "Disk space is low: {}%", 15);
    status!(err "Connection refused: {}", "127.0.0.1:8080");
    status!(info "Processing {} records", 42);

    // Formatted output
    cprint!(color::CYAN, "Progress: [{:<10}]", "███████");
    cprintln!(color::CYAN, " {}%", 70);

    println!("\nANSI color codes are embedded at compile time via macros.");
    println!("Each macro wraps format_args!() with escape sequences.");
}

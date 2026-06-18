/// Entry point for `file_organizer` — a CLI tool that organizes files
/// by extension into folders.
///
/// Usage: file_organizer --dir <path>
///        file_organizer --help

mod config;
mod scanner;
mod organizer;

use config::Config;
use scanner::Scanner;
use organizer::Organizer;

fn main() {
    let config = Config::from_args();

    if config.help {
        print_help();
        return;
    }

    let scanner = Scanner::new(&config.directory);
    let files = scanner.scan();
    let organizer = Organizer::new(&config.directory, files);
    let summary = organizer.organize();

    println!("{}", summary);
}

fn print_help() {
    println!("file_organizer — Organize files by extension into folders");
    println!();
    println!("USAGE:");
    println!("  file_organizer --dir <path>    Organize files in <path>");
    println!("  file_organizer --help           Show this help message");
    println!();
    println!("DESCRIPTION:");
    println!("  Scans the specified directory for files (non-recursively)");
    println!("  and moves each file into a subfolder named after its extension.");
    println!("  For example, 'readme.md' -> 'md/readme.md'.");
    println!();
    println!("OPTIONS:");
    println!("  --dir <path>    Target directory to organize");
    println!("  --help          Show this help and exit");
}

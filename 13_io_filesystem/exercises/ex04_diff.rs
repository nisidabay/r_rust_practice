// ex04_diff.rs — Compare two files line by line, print differences
// Usage: ./ex04_diff <file_a> <file_b>
// Shows lines that differ, with line numbers. Like `diff -u` simplified.

use std::env;
use std::fs;
use std::io;

fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().map(|l| l.to_string()).collect())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file_a> <file_b>", args.get(0).unwrap_or(&"ex04_diff".into()));
        std::process::exit(1);
    }

    let lines_a = read_lines(&args[1])?;
    let lines_b = read_lines(&args[2])?;

    let max_lines = lines_a.len().max(lines_b.len());
    let mut differences = 0;

    for i in 0..max_lines {
        match (lines_a.get(i), lines_b.get(i)) {
            (Some(a), Some(b)) => {
                if a != b {
                    println!("@@ line {} @@", i + 1);
                    println!("-{}", a);
                    println!("+{}", b);
                    differences += 1;
                }
            }
            (Some(a), None) => {
                println!("@@ line {} @@ (only in first)", i + 1);
                println!("-{}", a);
                differences += 1;
            }
            (None, Some(b)) => {
                println!("@@ line {} @@ (only in second)", i + 1);
                println!("+{}", b);
                differences += 1;
            }
            (None, None) => unreachable!(),
        }
    }

    if differences == 0 {
        println!("Files are identical.");
    } else {
        println!("\n{} difference(s) found.", differences);
    }

    Ok(())
}

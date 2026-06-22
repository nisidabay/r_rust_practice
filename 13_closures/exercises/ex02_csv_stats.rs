// ex02_csv_stats.rs — Read CSV lines, parse, calculate per-column stats with closures
//
// Input: CSV with header row, then data rows (comma separated, integers)
// Output: for each column, show min, max, average
//
// Example:
//   name,score,age
//   Alice,85,30
//   Bob,92,25
//   Charlie,78,35

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read and parse the header
    let header = match lines.next() {
        Some(Ok(line)) => line,
        _ => {
            eprintln!("Error: no header line found");
            return;
        }
    };
    let headers: Vec<&str> = header.split(',').collect();
    let ncols = headers.len();
    println!("Columns: {:?}", headers);

    // Collect all values per column. Skip non-numeric columns.
    let mut numeric_cols: Vec<Vec<i32>> = vec![Vec::new(); ncols];

    for line in lines {
        match line {
            Ok(text) => {
                let values: Vec<&str> = text.split(',').collect();
                for (i, val) in values.iter().enumerate() {
                    if i < ncols {
                        if let Ok(n) = val.trim().parse::<i32>() {
                            numeric_cols[i].push(n);
                        }
                    }
                }
            }
            Err(_) => continue,
        }
    }

    // Calculate stats using closures for each column
    for (i, col) in numeric_cols.iter().enumerate() {
        let name = headers.get(i).unwrap_or(&"?");

        // Use closures to compute stats
        let min = col.iter().min().copied();
        let max = col.iter().max().copied();
        let avg = if col.is_empty() {
            None
        } else {
            let sum: i32 = col.iter().sum();
            Some(sum as f64 / col.len() as f64)
        };

        // Print using closures for formatting
        let fmt_stat = |label: &str, val: Option<f64>| -> String {
            match val {
                Some(v) => format!("{}: {:.1}", label, v),
                None => format!("{}: N/A", label),
            }
        };

        println!(
            "  {}: {}",
            name,
            fmt_stat("avg", avg)
        );
        if let Some(mn) = min {
            println!("    min: {}, max: {}", mn, max.unwrap_or(mn));
        } else {
            println!("    (no numeric data)");
        }
    }
}

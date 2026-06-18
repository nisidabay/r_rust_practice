// ex02_csv_stats — Compute min, max, avg, stddev on CSV column using iterators
//
// A CSV-like string contains rows of "name,value". Parse the values,
// compute min, max, average, and population standard deviation using
// iterator chains.
//
// Run with: ./ex02_csv_stats [--csv "name,val;..."] or use built-in sample data

use std::env;

fn parse_csv(data: &str) -> Vec<f64> {
    // Split by ';' for rows, then by ',' for columns, take the second column
    data.split(';')
        .filter_map(|row| {
            let parts: Vec<&str> = row.split(',').collect();
            if parts.len() == 2 {
                parts[1].trim().parse::<f64>().ok()
            } else {
                None
            }
        })
        .collect()
}

fn compute_stats(values: &[f64]) -> (f64, f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }

    // Min and max via iterator
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    // Average
    let count = values.len() as f64;
    let sum: f64 = values.iter().sum();
    let avg = sum / count;

    // Population standard deviation (not sample)
    let variance: f64 = values.iter().map(|v| (v - avg).powi(2)).sum::<f64>() / count;
    let stddev = variance.sqrt();

    (min, max, avg, stddev)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let csv_data = if args.len() > 2 && args[1] == "--csv" {
        args[2].clone()
    } else {
        // Default sample data: "name,value;..."
        "Alice,85.5;Bob,92.3;Carol,78.0;Dave,88.1;Eve,95.7".to_string()
    };

    let values = parse_csv(&csv_data);
    let (min, max, avg, stddev) = compute_stats(&values);

    println!("CSV data parsed: {:?}", values);
    println!("Count:    {}", values.len());
    println!("Min:      {:.2}", min);
    println!("Max:      {:.2}", max);
    println!("Average:  {:.2}", avg);
    println!("Std Dev:  {:.4}", stddev);

    // Verify with default data
    if args.len() <= 2 {
        assert_eq!(values.len(), 5);
        assert!((avg - 87.92).abs() < 0.01, "Expected avg ~87.92, got {avg}");
        assert!((min - 78.0).abs() < 0.01, "Expected min 78.0, got {min}");
        assert!((max - 95.7).abs() < 0.01, "Expected max 95.7, got {max}");
        println!("Default data verification passed!");
    }
}

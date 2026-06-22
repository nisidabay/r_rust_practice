// ex04_bonus_cut.rs
// cut clone — extract columns from delimited text
//
// Flags:
//   -d <delimiter>   Field delimiter (default: tab)
//   -f <fields>      Comma-separated field numbers (1-indexed)
//
// Usage:
//   echo "a,b,c" | rustc ex04_bonus_cut.rs && ./ex04_bonus_cut -d , -f 1,3
//   echo -e "col1\tcol2\tcol3" | ./ex04_bonus_cut -f 2
//   cat data.csv | ./ex04_bonus_cut -d , -f 1,2

use std::collections::BTreeSet;
use std::env;
use std::io::{self, BufRead};

struct CutConfig {
    delimiter: String,
    fields: BTreeSet<usize>,
}

fn parse_fields(s: &str) -> Result<BTreeSet<usize>, String> {
    let mut fields = BTreeSet::new();
    for part in s.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let num: usize = part
            .parse()
            .map_err(|_| format!("Invalid field number: '{}'", part))?;
        if num == 0 {
            return Err("Field numbers are 1-indexed, got 0".to_string());
        }
        fields.insert(num);
    }
    if fields.is_empty() {
        return Err("No fields specified".to_string());
    }
    Ok(fields)
}

fn parse_args() -> Result<CutConfig, String> {
    let args: Vec<String> = env::args().collect();
    let mut delimiter = "\t".to_string();
    let mut fields: Option<BTreeSet<usize>> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-d" => {
                i += 1;
                if i < args.len() {
                    delimiter = args[i].clone();
                } else {
                    return Err("-d requires a delimiter argument".to_string());
                }
            }
            "-f" => {
                i += 1;
                if i < args.len() {
                    fields = Some(parse_fields(&args[i])?);
                } else {
                    return Err("-f requires a field list argument".to_string());
                }
            }
            f => return Err(format!("Unknown flag: '{}'", f)),
        }
        i += 1;
    }

    let fields = fields.ok_or_else(|| "Usage: ex04_bonus_cut -d <delimiter> -f <fields>".to_string())?;

    Ok(CutConfig { delimiter, fields })
}

fn main() -> Result<(), String> {
    let config = parse_args()?;

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line_result in reader.lines() {
        let line = line_result.map_err(|e| format!("Read error: {}", e))?;

        let columns: Vec<&str> = line.split(&config.delimiter).collect();
        let selected: Vec<&str> = config
            .fields
            .iter()
            .filter_map(|&f| {
                if f > 0 && f <= columns.len() {
                    Some(columns[f - 1])
                } else {
                    None
                }
            })
            .collect();

        println!("{}", selected.join(&config.delimiter));
    }

    Ok(())
}

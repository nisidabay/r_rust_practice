// 08_try_blocks — try {} blocks, collecting Results
//
// try {} blocks (experimental, #![feature(try_blocks)]) let you use ?
// inside a block that evaluates to a Result. Since they're still
// experimental in Rust 1.92, we demonstrate equivalent patterns:
//
//   1. Collecting Results from an iterator (stable)
//   2. Using closures to simulate try blocks
//   3. The `anyhow`/`eyre` pattern with helper functions

#![allow(unused)]

fn main() {
    // --- 1. Pattern: early returns with a closure ---
    // Use a closure that you immediately call to simulate a try block:
    let result: Result<i32, &str> = (|| {
        let x: i32 = "42".parse().map_err(|_| "parse failed")?;
        let y: i32 = "10".parse().map_err(|_| "parse failed")?;
        Ok(x + y)
    })();

    match result {
        Ok(v) => println!("closure sum: {v}"),
        Err(e) => println!("closure error: {e}"),
    }

    // --- 2. With early return on error ---
    let result: Result<i32, &str> = (|| {
        let x: i32 = "abc".parse().map_err(|_| "parse failed")?; // fails here
        let y: i32 = "10".parse().map_err(|_| "parse failed")?;
        Ok(x + y) // never reached
    })();

    println!("closure with parse error: {:?}", result);

    // --- 3. Collecting Results from an iterator (stable) ---
    // This is the most common "try block" pattern — collect all results
    // into a single Result.
    let nums: Vec<&str> = vec!["1", "2", "3", "4", "5"];
    let parsed: Result<Vec<i32>, _> = nums
        .iter()
        .map(|s| s.parse::<i32>())
        .collect(); // collects all or first error

    println!("Collect results (all ok): {:?}", parsed);

    let nums2: Vec<&str> = vec!["1", "bad", "3"];
    let parsed2: Result<Vec<i32>, _> = nums2
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();

    println!("Collect results (some bad): {:?}", parsed2);

    // --- 4. Helper function pattern (most Rustic) ---
    #[derive(Debug)]
    struct Config {
        host: String,
        port: u16,
    }

    fn build_config(host: Option<String>, port_str: &str) -> Result<Config, String> {
        let host = host.ok_or("host is required")?;
        let port: u16 = port_str.parse().map_err(|_| "invalid port")?;
        Ok(Config { host, port })
    }

    let cfg1 = build_config(Some("localhost".into()), "8080");
    println!("Config ok: {:?}", cfg1);

    let cfg2 = build_config(None, "8080");
    println!("Config missing host: {:?}", cfg2);

    let cfg3 = build_config(Some("localhost".into()), "not_a_port");
    println!("Config bad port: {:?}", cfg3);

    // --- 5. Mixed error types with Box<dyn Error> ---
    fn read_and_parse(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let num: i32 = content.trim().parse()?;
        Ok(num)
    }

    match read_and_parse("/nonexistent") {
        Ok(v) => println!("Read: {v}"),
        Err(e) => println!("Read error: {e}"),
    }

    // --- 6. Using combinators instead of try blocks ---
    // Equivalent to a try block that maps:
    let host = Some("localhost".to_string());
    let port_str = "8080";

    let cfg: Result<Config, String> = host
        .ok_or_else(|| "host is required".to_string())
        .and_then(|h| {
            port_str
                .parse::<u16>()
                .map_err(|_| "invalid port".to_string())
                .map(|p| Config { host: h, port: p })
        });

    println!("Combinator config: {:?}", cfg);

    println!("--- 08_try_blocks done ---");
}

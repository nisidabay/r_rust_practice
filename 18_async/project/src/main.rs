// fetch_multi — A CLI tool that fetches multiple URLs concurrently.
//
// Usage:
//   cargo run -- urls.txt
//   cargo run -- --help
//
// urls.txt contains one URL per line.
// All URLs are fetched concurrently using thread::spawn.
//
// In production you'd use an async runtime (tokio) with an async HTTP
// client (reqwest). Here we use ureq (blocking) wrapped in threads,
// which demonstrates the same concurrency pattern without needing a
// full async runtime. The thread::spawn + join pattern mirrors
// tokio::spawn + join_all in spirit.

use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use ureq::Agent;

/// Fetch a single URL using ureq (blocking).
/// Returns the URL and a summary string (success or error).
fn fetch_url(agent: &Agent, url: &str) -> (String, String) {

    let start = Instant::now();
    match agent.get(url).call() {
        Ok(response) => {
            let mut body = Vec::new();
            let _ = response.into_reader().read_to_end(&mut body);
            let len = body.len();
            (
                url.to_string(),
                format!("{} bytes fetched in {:?}", len, start.elapsed()),
            )
        }
        Err(e) => (
            url.to_string(),
            format!("Error after {:?}: {e}", start.elapsed()),
        ),
    }
}

/// Read URLs from a file (one per line, skipping blanks and comments).
fn read_urls(path: &PathBuf) -> Vec<String> {
    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Error reading {path:?}: {e}");
        std::process::exit(1);
    });

    content
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.to_string())
        .collect()
}

fn print_usage(program: &str) -> ! {
    eprintln!("Usage: {program} <urls_file>");
    eprintln!();
    eprintln!("Reads URLs from a file (one per line) and fetches all of them concurrently.");
    eprintln!("Output shows timing for each URL and total elapsed time.");
    eprintln!();
    eprintln!("URL file format (one per line):");
    eprintln!("  https://example.com");
    eprintln!("  https://httpbin.org/get");
    eprintln!("  # lines starting with # are comments");
    eprintln!("  blank lines are ignored");
    std::process::exit(1);
}

fn main() {
    // --- Parse arguments ---
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || args[1] == "--help" || args[1] == "-h" {
        print_usage(&args[0]);
    }

    let url_file = PathBuf::from(&args[1]);
    let urls = read_urls(&url_file);

    if urls.is_empty() {
        eprintln!("No URLs found in {url_file:?}");
        std::process::exit(1);
    }

    println!("Fetching {} URLs concurrently...\n", urls.len());
    let total_start = Instant::now();

    // --- Create an agent with timeouts ---
    let agent = Arc::new(ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(30))
        .build());

    // --- Spawn one thread per URL ---
    // This is the concurrent-execution equivalent of:
    //
    //   let tasks: Vec<_> = urls.iter().map(|url| {
    //       tokio::spawn(async { fetch_url(url).await })
    //   }).collect();
    //   let results = join_all(tasks).await;

    let handles: Vec<_> = urls
        .iter()
        .map(|url| {
            let url = url.clone();
            let a = Arc::clone(&agent);
            std::thread::spawn(move || fetch_url(&a, &url))
        })
        .collect();

    // --- Collect results (like .await on all futures) ---
    let mut results: Vec<(String, String)> =
        handles.into_iter().map(|h| h.join().unwrap()).collect();

    // --- Sort results by URL for stable output ---
    results.sort_by(|a, b| a.0.cmp(&b.0));

    // --- Classify and print results ---
    let success_count = results.iter().filter(|r| !r.1.starts_with("Error")).count();
    let error_count = results.len() - success_count;

    for (url, msg) in &results {
        if msg.starts_with("Error") {
            println!("  \u{274c} {url} \u{2014} {msg}");
        } else {
            println!("  \u{2705} {url} \u{2014} {msg}");
        }
    }

    let total_elapsed = total_start.elapsed();
    println!();
    println!("Summary: {success_count} succeeded, {error_count} failed in {total_elapsed:?}");

    if error_count > 0 {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_urls() {
        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        write!(
            tmpfile,
            "https://example.com\nhttps://httpbin.org/get\n# comment\n\nhttps://google.com"
        )
        .unwrap();

        let urls = read_urls(&tmpfile.path().to_path_buf());
        assert_eq!(urls.len(), 3);
        assert_eq!(urls[0], "https://example.com");
        assert_eq!(urls[1], "https://httpbin.org/get");
        assert_eq!(urls[2], "https://google.com");
    }

    #[test]
    fn test_fetch_url_success() {
        let agent = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(10))
            .build();
        let (url, msg) = fetch_url(&agent, "https://example.com");
        assert_eq!(url, "https://example.com");
        assert!(!msg.starts_with("Error"), "Should fetch example.com: {msg}");
        assert!(msg.contains("bytes fetched"), "Should report byte count");
    }

    #[test]
    fn test_fetch_url_failure() {
        let agent = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(2))
            .build();
        let (url, msg) = fetch_url(&agent, "https://nonexistent.invalid.url.xyz");
        assert_eq!(url, "https://nonexistent.invalid.url.xyz");
        assert!(msg.starts_with("Error"), "Should fail on invalid URL: {msg}");
    }

    #[test]
    fn test_empty_urls_file() {
        let tmpfile = tempfile::NamedTempFile::new().unwrap();
        let urls = read_urls(&tmpfile.path().to_path_buf());
        assert!(urls.is_empty());
    }
}

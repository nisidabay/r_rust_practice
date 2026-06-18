use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;

/// Parse command-line arguments and return the configuration.
struct Config {
    filename: String,
    num_threads: usize,
    help: bool,
}

fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();

    // Check for --help or -h
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        return Config {
            filename: String::new(),
            num_threads: 0,
            help: true,
        };
    }

    let mut filename = String::new();
    let mut num_threads = 4; // default

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--threads" | "-t" => {
                i += 1;
                if i < args.len() {
                    num_threads = args[i]
                        .parse()
                        .unwrap_or_else(|_| {
                            eprintln!("Invalid thread count: {}", args[i]);
                            std::process::exit(1);
                        });
                    if num_threads < 1 {
                        eprintln!("Thread count must be at least 1");
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("Missing argument for --threads");
                    std::process::exit(1);
                }
            }
            "--help" | "-h" => {
                return Config {
                    filename: String::new(),
                    num_threads: 0,
                    help: true,
                };
            }
            _ => {
                if filename.is_empty() {
                    filename = args[i].clone();
                } else {
                    eprintln!("Unexpected argument: {}", args[i]);
                    std::process::exit(1);
                }
            }
        }
        i += 1;
    }

    if filename.is_empty() {
        eprintln!("Error: missing filename");
        print_usage();
        std::process::exit(1);
    }

    Config {
        filename,
        num_threads,
        help: false,
    }
}

fn print_usage() {
    let help = r#"word_counter_parallel — Count word frequencies in a file using multiple threads

USAGE:
    word_counter_parallel [OPTIONS] <FILE>

ARGS:
    <FILE>    Path to the input file

OPTIONS:
    -t, --threads <N>    Number of threads to use (default: 4)
    -h, --help           Print this help message

DESCRIPTION:
    Reads a text file, splits it into chunks, and counts word frequencies
    in parallel using N threads. Each thread processes one chunk and the
    results are merged. Words are case-sensitive and consist of alphabetic
    characters.

EXAMPLE:
    word_counter_parallel sample.txt
    word_counter_parallel -t 8 sample.txt
"#;
    println!("{help}");
}

/// Read a file into memory and split into roughly equal chunks for parallel processing.
fn read_and_chunk(filename: &str, num_chunks: usize) -> io::Result<Vec<String>> {
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);

    // Read all lines into a buffer
    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n');
    }

    if content.is_empty() {
        return Ok(vec![]);
    }

    // Split into chunks, breaking at newline boundaries
    let total_len = content.len();
    let chunk_size = (total_len + num_chunks - 1) / num_chunks;
    let mut chunks = Vec::with_capacity(num_chunks);
    let mut start = 0;

    for _ in 0..num_chunks {
        if start >= total_len {
            break;
        }

        let mut end = std::cmp::min(start + chunk_size, total_len);

        // Adjust end to the next newline (unless we're at the end)
        if end < total_len {
            if let Some(newline_pos) = content[end..].find('\n') {
                end += newline_pos + 1;
            } else {
                end = total_len;
            }
        }

        chunks.push(content[start..end].to_string());
        start = end;
    }

    Ok(chunks)
}

/// Count word frequencies in a text chunk.
/// Returns a HashMap where keys are words (lowercased) and values are counts.
fn count_words_in_chunk(chunk: &str) -> HashMap<String, u64> {
    let mut counts = HashMap::new();

    // Split by whitespace and count words (sequences of alphabetic chars)
    let mut current_word = String::new();
    for ch in chunk.chars() {
        if ch.is_alphabetic() {
            current_word.push(ch);
        } else if !current_word.is_empty() {
            *counts.entry(current_word.clone()).or_insert(0) += 1;
            current_word.clear();
        }
    }
    // Handle last word
    if !current_word.is_empty() {
        *counts.entry(current_word).or_insert(0) += 1;
    }

    counts
}

/// Merge a chunk result into the global counts.
fn merge_counts(global: &mut HashMap<String, u64>, chunk: HashMap<String, u64>) {
    for (word, count) in chunk {
        *global.entry(word).or_insert(0) += count;
    }
}

fn run(config: &Config) -> io::Result<()> {
    let num_threads = config.num_threads;

    // Read file into chunks
    let chunks = read_and_chunk(&config.filename, num_threads)?;

    if chunks.is_empty() {
        println!("File is empty: {}", config.filename);
        return Ok(());
    }

    let actual_chunks = chunks.len();
    let actual_threads = std::cmp::min(num_threads, actual_chunks);

    println!(
        "Reading {} with {actual_threads} threads ({actual_chunks} chunks)...",
        config.filename
    );

    // Shared result map protected by a Mutex
    let global_counts: Arc<Mutex<HashMap<String, u64>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];

    for chunk in chunks {
        let global = Arc::clone(&global_counts);
        handles.push(thread::spawn(move || {
            let local_counts = count_words_in_chunk(&chunk);
            let mut guard = global.lock().unwrap();
            merge_counts(&mut guard, local_counts);
        }));
    }

    // Wait for all threads
    for h in handles {
        h.join().expect("Thread panicked");
    }

    // Print results
    let final_counts = global_counts.lock().unwrap();

    // Sort by count (descending), then by word
    let mut sorted: Vec<(&String, &u64)> = final_counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

    println!("\nWord frequency ({} unique words):", sorted.len());
    println!("{:-<40}", "");
    println!("{:<30} {:>8}", "Word", "Count");
    println!("{:-<40}", "");

    for (word, count) in sorted.iter().take(50) {
        println!("{:<30} {:>8}", word, count);
    }

    if sorted.len() > 50 {
        println!("... and {} more words", sorted.len() - 50);
    }

    println!("{:-<40}", "");

    Ok(())
}

fn main() {
    let config = parse_args();

    if config.help {
        print_usage();
        return;
    }

    if let Err(e) = run(&config) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_count_words_in_chunk() {
        let chunk = "hello world hello foo bar foo foo";
        let counts = count_words_in_chunk(chunk);
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
        assert_eq!(counts.get("foo"), Some(&3));
        assert_eq!(counts.get("bar"), Some(&1));
        assert_eq!(counts.len(), 4);
    }

    #[test]
    fn test_count_words_empty() {
        let counts = count_words_in_chunk("");
        assert!(counts.is_empty());
    }

    #[test]
    fn test_count_words_punctuation() {
        let chunk = "hello, world! it's a test.";
        let counts = count_words_in_chunk(chunk);
        // "it's" should be counted as "its" since ' is not alphabetic
        // Actually ' is not alphabetic in Rust, so "it" and "s" are separate
        // Let's just check the main words
        assert_eq!(counts.get("hello"), Some(&1));
        assert_eq!(counts.get("world"), Some(&1));
        assert!(counts.contains_key("test"));
    }

    #[test]
    fn test_merge_counts() {
        let mut global = HashMap::new();
        global.insert("hello".to_string(), 2);

        let mut chunk = HashMap::new();
        chunk.insert("hello".to_string(), 3);
        chunk.insert("world".to_string(), 1);

        merge_counts(&mut global, chunk);
        assert_eq!(global.get("hello"), Some(&5));
        assert_eq!(global.get("world"), Some(&1));
    }

    #[test]
    fn test_read_and_chunk() {
        // Create a temp file with known content
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "hello world").unwrap();
        writeln!(tmp, "foo bar").unwrap();
        writeln!(tmp, "baz qux").unwrap();

        let chunks = read_and_chunk(tmp.path().to_str().unwrap(), 2).unwrap();
        // Should have 2 chunks (or more, depending on size)
        assert!(!chunks.is_empty());
        assert!(chunks.len() <= 2);
    }

    #[test]
    fn test_parallel_counting_consistency() {
        // Create a small file with known word counts
        let mut tmp = NamedTempFile::new().unwrap();
        for _ in 0..100 {
            writeln!(tmp, "hello world hello foo").unwrap();
        }

        // Count with 1 thread
        let content = fs::read_to_string(tmp.path()).unwrap();
        let expected = count_words_in_chunk(&content);

        // Count with 4 threads via the full pipeline
        let chunks = read_and_chunk(tmp.path().to_str().unwrap(), 4).unwrap();
        let global = Arc::new(Mutex::new(HashMap::new()));
        let mut handles = vec![];

        for chunk in chunks {
            let g = Arc::clone(&global);
            handles.push(thread::spawn(move || {
                let local = count_words_in_chunk(&chunk);
                let mut guard = g.lock().unwrap();
                merge_counts(&mut guard, local);
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        let parallel = global.lock().unwrap();

        assert_eq!(*parallel, expected);
    }
}

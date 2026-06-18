// ex04_inverted_index.rs — Build inverted index from sentences (BONUS)
//
// An inverted index maps each word → list of sentence IDs it appears in.
// Given a list of sentences, build the index and support phrase search.

use std::collections::{HashMap, HashSet};

/// Build an inverted index: word -> set of sentence IDs
fn build_index(sentences: &[&str]) -> HashMap<String, HashSet<usize>> {
    let mut index: HashMap<String, HashSet<usize>> = HashMap::new();

    for (sid, sentence) in sentences.iter().enumerate() {
        for word in sentence.split_whitespace() {
            let clean: String = word
                .chars()
                .filter(|c| c.is_alphanumeric())
                .flat_map(|c| c.to_lowercase())
                .collect();
            if !clean.is_empty() {
                index.entry(clean).or_default().insert(sid);
            }
        }
    }

    index
}

/// Find sentences containing ALL words in a query (AND search)
fn search(query: &str, index: &HashMap<String, HashSet<usize>>) -> Vec<usize> {
    let words: Vec<&str> = query.split_whitespace().collect();
    if words.is_empty() {
        return vec![];
    }

    let mut results: Option<HashSet<usize>> = None;

    for w in words {
        let clean: String = w
            .chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(|c| c.to_lowercase())
            .collect();

        match index.get(&clean) {
            Some(sentences) => {
                if let Some(acc) = &mut results {
                    acc.retain(|s| sentences.contains(s));
                } else {
                    results = Some(sentences.clone());
                }
            }
            None => return vec![],
        }
    }

    results.map(|r| {
        let mut v: Vec<_> = r.into_iter().collect();
        v.sort();
        v
    }).unwrap_or_default()
}

fn main() {
    let sentences = vec![
        "The quick brown fox jumps over the lazy dog.",
        "A quick brown dog outruns a lazy fox.",
        "Rust is a fast and safe systems programming language.",
        "The fox and the dog are both fast animals.",
    ];

    let index = build_index(&sentences);

    println!("Inverted index:");
    let mut words: Vec<_> = index.keys().collect();
    words.sort();
    for w in &words {
        let ids: Vec<usize> = index[*w].iter().copied().collect();
        println!("  \"{w}\" → sentences {ids:?}");
    }

    // Test searches
    println!();
    for query in &["fox dog", "Rust", "quick lazy", "brown fox"] {
        let results = search(query, &index);
        println!("search(\"{query}\") → sentences {results:?}");
    }

    // Assertions
    assert_eq!(search("fox dog", &index), vec![0, 1, 3]);
    assert_eq!(search("Rust", &index), vec![2]);
    assert_eq!(search("lazy fox", &index), vec![0, 1]);
    assert!(search("python", &index).is_empty());

    println!("\nAll assertions passed.");
}

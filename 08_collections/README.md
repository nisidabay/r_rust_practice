# 08 Collections — Vec, HashMap, HashSet, and iterators

> Collections are where you put more than one value. Rust's standard library gives you four you'll use daily: Vec (ordered, indexable), HashMap (key-value), HashSet (unique items), and iterators (transformation pipeline).

---

## Cheatsheet

```rust
// Vec<T> — growable array
let mut v = vec![1, 2, 3];
v.push(4);             // append
v.pop();               // remove last → Option<T>
v.insert(1, 99);       // insert at index
v.remove(0);           // remove at index
v.get(5);              // safe access → Option<&T>
v.resize(10, 0);       // fill to length with default

// HashMap<K, V> — key-value store
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", 42);
map.get("key");                        // Option<&V>
map.entry("new").or_insert(0);         // insert if missing
map.contains_key("key");               // bool
map.remove("key");                     // Option<V>

// HashSet<T> — unique values
use std::collections::HashSet;
let mut set = HashSet::new();
set.insert("apple");
set.contains("apple");                 // true
a.union(&b);                           // iterator of elements in a OR b
a.intersection(&b);                    // iterator of elements in a AND b
a.difference(&b);                      // iterator of elements in a NOT b

// String building from collections
words.join(", ");                      // "a, b, c"
pieces.concat();                       // "abc"
let s: String = chars.iter().collect();

// Iterator methods
vec.iter().map(|x| x * 2).collect::<Vec<_>>();
vec.iter().filter(|x| *x % 2 == 0);
vec.iter().fold(0, |acc, x| acc + x);  // like reduce
vec.iter().sum::<i32>();
vec.iter().any(|x| *x > 5);
vec.iter().all(|x| *x < 10);
```

---

## Concepts (in order, compile and run each)

```bash
rustc --edition 2021 concept/01_vec.rs -o /tmp/check && /tmp/check
rustc --edition 2021 concept/02_hashmap.rs -o /tmp/check && /tmp/check
rustc --edition 2021 concept/03_hashset.rs -o /tmp/check && /tmp/check
rustc --edition 2021 concept/04_string_collect.rs -o /tmp/check && /tmp/check
rustc --edition 2021 concept/05_iter_methods.rs -o /tmp/check && /tmp/check
```

| # | File | Teaches | In 10 seconds |
|---|------|---------|---------------|
| 01 | `01_vec.rs` | Vec create, push, pop, insert, remove, get | Growable, indexable, safe access with `.get()` |
| 02 | `02_hashmap.rs` | HashMap insert, get, entry, or_insert, remove | Key-value lookups, Entry API for upsert |
| 03 | `03_hashset.rs` | HashSet insert, contains, union, intersection, difference | Fast membership testing, set operations |
| 04 | `04_string_collect.rs` | join, collect, concat, format! | Build strings from collections |
| 05 | `05_iter_methods.rs` | map, filter, collect, fold, sum, any, all | Transformation pipeline without loops |

---

## Exercises

```bash
cd exercises
make run    # compile + run all exercises
```

| # | File | What it does | Concepts used |
|---|------|-------------|---------------|
| 01 | `ex01_word_freq.rs` | Count word frequency from stdin, print top 5 | HashMap, iterators, sorting |
| 02 | `ex02_merge_dedup.rs` | Merge two comma-separated lists, deduplicate, sort | BTreeSet, split, chain |
| 03 | `ex03_palindrome.rs` | Check palindrome (ignore case + spaces) | Vec<char>, two-pointer |
| 🏆 | `ex04_bonus_inverted_index.rs` | Map words → line numbers they appear on | HashMap<String, Vec<usize>> |

---

## Project — Frequency Counter

Counts character, word, and line frequency from stdin. Prints histogram-style bars.

```bash
cd project
echo "hello world hello" | cargo run --
echo "foo foo bar" | cargo run -- --sort freq --min-count 2
```

**Flags:**
- `--sort alpha` — alphabetical order (default)
- `--sort freq` — sort by frequency (descending)
- `--min-count N` — only show items with count ≥ N

When you see a histogram of word frequencies, think:
**"HashMap + iterators = everything I need for data analysis."**

---

## What's next?

Group 09 `09_patterns`: match on literals, destructuring, if let, ref patterns — unpacking data safely.

// 05_path_buf.rs — Path, PathBuf, path manipulation
// Uses: std::path::{Path, PathBuf}, fs::metadata, join, parent, extension

use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // --- PathBuf vs Path ---
    // PathBuf: owned, mutable path string (like String)
    // Path: borrowed, immutable slice (like &str)

    // Creating PathBuf values
    let mut pb = PathBuf::new();
    pb.push("home");
    pb.push("user");
    pb.push("documents");
    pb.push("file.txt");
    println!("PathBuf: {:?}", pb);
    // /home/user/documents/file.txt

    // --- join (doesn't mutate, returns new PathBuf) ---
    let base = Path::new("/home/user");
    let full = base.join("projects").join("rust").join("main.rs");
    println!("join: {:?}", full);
    // /home/user/projects/rust/main.rs

    // --- with_extension ---
    let renamed = full.with_extension("py");
    println!("with_extension: {:?}", renamed);
    // /home/user/projects/rust/main.py

    // --- parent (returns Option<&Path>) ---
    match full.parent() {
        Some(p) => println!("parent: {:?}", p),
        None => println!("no parent (root?)"),
    }
    // /home/user/projects/rust

    // --- ancestors (iterates parents upward) ---
    println!("=== ancestors ===");
    for (i, ancestor) in full.ancestors().enumerate() {
        println!("  {}: {:?}", i, ancestor);
    }

    // --- file_name (returns Option<&OsStr>) ---
    if let Some(name) = full.file_name() {
        println!("file_name: {:?}", name);
    }

    // --- file_stem (name without extension) ---
    if let Some(stem) = full.file_stem() {
        println!("file_stem: {:?}", stem);
    }

    // --- extension ---
    if let Some(ext) = full.extension() {
        println!("extension: {:?}", ext);
    }

    // --- is_absolute / is_relative ---
    println!("is_absolute: {}", full.is_absolute());
    println!("is_relative: {}", Path::new("relative/path").is_relative());

    // --- exists / is_file / is_dir ---
    // These filesystem checks require the path to actually exist.
    let p = Path::new("/tmp");
    println!("/tmp exists: {}", p.exists());
    println!("/tmp is dir: {}", p.is_dir());

    let f = Path::new("/etc/passwd");
    println!("/etc/passwd is_file: {}", f.is_file());

    // --- Conversion: &str -> Path/PathBuf ---
    let _from_str: &Path = Path::new("/usr/bin/env");
    let from_string: PathBuf = PathBuf::from("/usr/bin/env");
    let to_str = from_string.to_str().unwrap_or("<non-utf8>");
    println!("as str: {}", to_str);

    // --- Display ---
    // Path::display() gives a lossy Display impl
    println!("display: {}", full.display());

    // --- Real filesystem operations ---
    // current_dir, canonicalize
    match std::env::current_dir() {
        Ok(cwd) => println!("current_dir: {:?}", cwd),
        Err(e) => eprintln!("could not get cwd: {}", e),
    }

    match fs::canonicalize("/tmp") {
        Ok(abs) => println!("canonicalize /tmp: {:?}", abs),
        Err(e) => eprintln!("canonicalize failed: {}", e),
    }
}

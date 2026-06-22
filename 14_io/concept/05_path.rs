// 05_path.rs — Path and PathBuf for filesystem path handling
//
// Path is a borrowed slice (&Path, like &str), PathBuf is an owned path (like String).
// Together they provide cross-platform path manipulation without string concatenation.

use std::path::Path;
use std::fs;

fn main() {
    // Creating paths: Path::new for borrowed, join for building
    let base = Path::new("/home/user/docs");
    let file_path = base.join("report.txt");  // PathBuf
    println!("Joined path: {:?}", file_path);
    // On Windows this would use backslashes — Path is cross-platform

    // Check if a path exists and what it is
    let current = Path::new(".");
    println!("\nCurrent dir '.' exists: {}", current.exists());
    println!("'.' is a directory: {}", current.is_dir());
    println!("'.' is a file: {}", current.is_file());

    // Check this file specifically
    let this_file = Path::new("05_path.rs");
    println!("\n'05_path.rs' exists: {}", this_file.exists());
    println!("'05_path.rs' is file: {}", this_file.is_file());

    // Path components: extension, file_name, parent, file_stem
    println!("\n--- Path component analysis ---");
    let p = Path::new("/home/user/docs/report.txt.bak");

    println!("Full path: {:?}", p);
    println!("File name: {:?}", p.file_name());       // Some("report.txt.bak")
    println!("File stem: {:?}", p.file_stem());      // Some("report.txt") — no last extension
    println!("Extension: {:?}", p.extension());       // Some("bak")
    println!("Parent:    {:?}", p.parent());           // Some("/home/user/docs")
    println!("Ancestors:");
    for ancestor in p.ancestors() {
        println!("  {:?}", ancestor);
    }

    // canonicalize: resolve .. and symlinks to absolute path
    match fs::canonicalize(".") {
        Ok(abs) => println!("\nCanonical path of '.': {:?}", abs),
        Err(e) => eprintln!("Canonicalize error: {}", e),
    }

    // Path operations don't check the filesystem (except exists/is_file/is_dir)
    let non_existent = Path::new("/nonexistent/path/file.txt");
    println!("\nNon-existent path exists: {}", non_existent.exists());
    println!("Parent of non-existent: {:?}", non_existent.parent());

    // Converting between Path and &str
    let path_str = "data/config.json";
    let path_obj = Path::new(path_str);
    println!("\nFrom str: {:?}", path_obj);
    println!("To str: {:?}", path_obj.to_str());  // Option<&str>

    // Iterating over path components
    let components: Vec<&str> = path_obj.components()
        .map(|c| c.as_os_str().to_str().unwrap_or("?"))
        .collect();
    println!("Components: {:?}", components);

    println!("\nPath/PathBuf: safe, cross-platform filesystem paths.");
}

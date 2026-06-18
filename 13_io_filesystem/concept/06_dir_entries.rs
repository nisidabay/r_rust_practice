// 06_dir_entries.rs — Reading directories, walking trees
// Uses: fs::read_dir, manual recursion (WalkDir-style with stdlib only)

use std::fs;
use std::io;
use std::path::Path;

/// Walk a directory tree recursively (stdlib only, no external crate).
fn walk_dir(path: &Path, depth: usize) -> io::Result<()> {
    // Read the directory
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();

        // Indent based on depth
        let indent = "  ".repeat(depth);

        // Get file name
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();

        // Print entry
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            println!("{}[DIR]  {}/", indent, name_str);
            walk_dir(&entry_path, depth + 1)?;
        } else if file_type.is_file() {
            let metadata = entry.metadata()?;
            let size = metadata.len();
            println!("{}[FILE] {} ({} bytes)", indent, name_str, size);
        } else if file_type.is_symlink() {
            let target = fs::read_link(&entry_path)?;
            println!("{}[LINK] {} -> {:?}", indent, name_str, target);
        }
    }
    Ok(())
}

/// Count total items (files + dirs) in a tree.
fn count_entries(path: &Path) -> io::Result<usize> {
    let mut count = 1; // this entry itself
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                count += count_entries(&entry_path)?;
            } else {
                count += 1;
            }
        }
    }
    Ok(count)
}

/// List files matching a glob-like extension filter.
fn find_files_by_ext(path: &Path, ext: &str) -> io::Result<Vec<std::path::PathBuf>> {
    let mut results = Vec::new();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                results.extend(find_files_by_ext(&entry_path, ext)?);
            } else if let Some(extension) = entry_path.extension() {
                if extension == ext {
                    results.push(entry_path);
                }
            }
        }
    }
    Ok(results)
}

fn main() -> io::Result<()> {
    // Create a temp directory structure for demonstration
    let tmp = Path::new("/tmp/_demo_dir");
    let _ = fs::remove_dir_all(tmp); // clean slate
    fs::create_dir_all(tmp.join("sub1"))?;
    fs::create_dir_all(tmp.join("sub1").join("nested"))?;
    fs::create_dir_all(tmp.join("sub2"))?;
    fs::write(tmp.join("a.txt"), "hello")?;
    fs::write(tmp.join("sub1").join("b.rs"), "fn main() {}")?;
    fs::write(tmp.join("sub1").join("nested").join("c.txt"), "deep")?;
    fs::write(tmp.join("sub2").join("d.md"), "# title")?;

    // --- walk_dir ---
    println!("=== Walk tree ===");
    walk_dir(tmp, 0)?;

    // --- count_entries ---
    let count = count_entries(tmp)?;
    println!("\nTotal entries in tree: {}", count);

    // --- find_files_by_ext ---
    let txt_files = find_files_by_ext(tmp, "txt")?;
    println!("\n=== .txt files ===");
    for f in &txt_files {
        println!("  {:?}", f);
    }

    // --- Individual entry info ---
    println!("\n=== Individual entry ===");
    for entry in fs::read_dir(tmp)? {
        let entry = entry?;
        let path = entry.path();
        let ft = entry.file_type()?;
        println!("  {:?}  is_dir={}  is_file={}", path, ft.is_dir(), ft.is_file());
    }

    // Cleanup
    fs::remove_dir_all(tmp)?;

    Ok(())
}

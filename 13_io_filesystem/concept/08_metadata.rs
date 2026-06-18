// 08_metadata.rs — File metadata: type, size, permissions, timestamps
// Uses: fs::metadata, fs::symlink_metadata, Permissions, file_type inspection

use std::fs::{self, File};
use std::io::{self, Write};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;
use std::time::SystemTime;

fn main() -> io::Result<()> {
    let path = "/tmp/_demo_metadata.txt";

    // Create a file with some content
    let mut file = File::create(path)?;
    writeln!(file, "Hello, metadata!")?;
    file.sync_all()?; // ensure metadata is written

    // --- fs::metadata ---
    // Returns metadata for a path. Follows symlinks.
    // For symlinks themselves (not their targets), use fs::symlink_metadata.
    let meta = fs::metadata(path)?;
    println!("=== fs::metadata for {:?} ===", path);

    // File type
    println!("file_type.is_file: {}", meta.file_type().is_file());
    println!("file_type.is_dir: {}", meta.file_type().is_dir());
    println!("file_type.is_symlink: {}", meta.file_type().is_symlink());

    // Size in bytes
    println!("len: {} bytes", meta.len());

    // Permissions
    let perms = meta.permissions();
    println!("readonly: {}", perms.readonly());
    println!("permissions: {:o}", perms.mode() & 0o777); // unix only

    // Timestamps (Unix: seconds + nanoseconds since epoch)
    match meta.created() {
        Ok(t) => println!("created: {:?}", t),
        Err(e) => println!("created not available: {}", e),
    }
    match meta.modified() {
        Ok(t) => println!("modified: {:?}", t),
        Err(e) => println!("modified not available: {}", e),
    }
    match meta.accessed() {
        Ok(t) => println!("accessed: {:?}", t),
        Err(e) => println!("accessed not available: {}", e),
    }

    // --- Unix-specific extended metadata (via MetadataExt) ---
    #[cfg(unix)]
    {
        println!("\n=== Unix metadata ===");
        println!("uid: {}", meta.uid());
        println!("gid: {}", meta.gid());
        println!("dev: {}", meta.dev());
        println!("ino: {}", meta.ino());
        println!("mode: {:o}", meta.mode());
        println!("nlink: {}", meta.nlink());
        println!("rdev: {}", meta.rdev());
        println!("blksize: {}", meta.blksize());
        println!("blocks: {}", meta.blocks());
    }

    // --- symlink_metadata ---
    // On a regular file, same as metadata.
    // On a symlink, reports the symlink itself, not the target.
    let sym_path = Path::new("/tmp/_demo_metadata_link");
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(path, sym_path)?;
        let sym_meta = fs::symlink_metadata(sym_path)?;
        println!("\nsymlink_metadata for symlink:");
        println!("  is_symlink: {}", sym_meta.file_type().is_symlink());
        println!("  len: {} (symlink size)", sym_meta.len());
        fs::remove_file(sym_path)?;
    }

    // --- is_file / is_dir convenience methods on Path ---
    println!("\n=== Path convenience methods ===");
    println!("/tmp is_dir: {}", Path::new("/tmp").is_dir());
    println!("/etc/passwd is_file: {}", Path::new("/etc/passwd").is_file());
    println!("/nonexistent exists: {}", Path::new("/nonexistent").exists());

    // --- Comparing timestamps ---
    let now = SystemTime::now();
    let modified = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    if let Ok(diff) = now.duration_since(modified) {
        println!("\nFile modified {} seconds ago", diff.as_secs());
    }

    // Cleanup
    fs::remove_file(path)?;

    Ok(())
}

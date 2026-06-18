/// Scanner module — traverses the target directory for files (non-recursive)

use std::fs;
use std::path::Path;

/// Scans a directory for regular files (ignores subdirectories)
pub struct Scanner {
    directory: String,
}

impl Scanner {
    pub fn new(directory: &str) -> Self {
        Self {
            directory: directory.to_string(),
        }
    }

    /// Returns a list of (file_name, extension) pairs found in the directory.
    /// Excludes hidden files (starting with '.') and directories.
    pub fn scan(&self) -> Vec<(String, String)> {
        let mut files = Vec::new();

        let dir = Path::new(&self.directory);
        if !dir.is_dir() {
            eprintln!("Warning: '{}' is not a directory or doesn't exist", self.directory);
            return files;
        }

        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error reading directory '{}': {}", self.directory, e);
                return files;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // Skip hidden files
                    if name.starts_with('.') {
                        continue;
                    }
                    let ext = path
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("noext")
                        .to_lowercase();
                    files.push((name.to_string(), ext));
                }
            }
        }

        files
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_nonexistent_dir() {
        let scanner = Scanner::new("/nonexistent/path/xyz123");
        let files = scanner.scan();
        assert!(files.is_empty());
    }
}

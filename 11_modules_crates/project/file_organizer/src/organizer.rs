/// Organizer module — moves files into extension-based folders

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Organizes a list of files by extension into subfolders
pub struct Organizer {
    directory: String,
    files: Vec<(String, String)>,
}

impl Organizer {
    pub fn new(directory: &str, files: Vec<(String, String)>) -> Self {
        Self {
            directory: directory.to_string(),
            files,
        }
    }

    /// Moves each file into a subfolder named after its extension.
    /// Returns a summary string of what was done.
    pub fn organize(&self) -> String {
        let mut summary = String::new();
        let mut by_ext: BTreeMap<String, Vec<&str>> = BTreeMap::new();

        for (name, ext) in &self.files {
            by_ext.entry(ext.clone()).or_default().push(name);
        }

        if by_ext.is_empty() {
            return "No files found to organize.".to_string();
        }

        summary.push_str(&format!("Organizing {} files in '{}'...\n", self.files.len(), self.directory));

        for (ext, names) in &by_ext {
            // Create the extension folder
            let ext_dir = Path::new(&self.directory).join(ext);
            if !ext_dir.exists() {
                if let Err(e) = fs::create_dir_all(&ext_dir) {
                    summary.push_str(&format!("  Error creating dir '{}': {}\n", ext, e));
                    continue;
                }
            }

            // Move each file into the folder
            for name in names {
                let src = Path::new(&self.directory).join(name);
                let dst = ext_dir.join(name);
                if dst.exists() {
                    summary.push_str(&format!("  SKIP '{}' -> '{}' (target exists)\n", name, ext));
                } else if let Err(e) = fs::rename(&src, &dst) {
                    summary.push_str(&format!("  ERROR moving '{}': {}\n", name, e));
                } else {
                    summary.push_str(&format!("  MOVED '{}' -> '{}'\n", name, ext));
                }
            }
        }

        summary.push_str("Done.");
        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organizer_empty_files() {
        let org = Organizer::new(".", vec![]);
        let result = org.organize();
        assert_eq!(result, "No files found to organize.");
    }
}

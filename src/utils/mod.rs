pub mod input;
use glob::Pattern;
use std::fs;
use std::path::Path;

pub enum SearchDirection {
    Parent,
    Child,
}

pub fn recursively_check_for_file(
    directory: &str,
    file_pattern: &str,
    levels: usize,
    direction: SearchDirection,
) -> Option<String> {
    let mut current_dir = directory.to_string();
    let pattern = Pattern::new(file_pattern).ok()?;

    for _ in 0..=levels {
        // Check if file exists in current directory
        if let Ok(entries) = fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    if pattern.matches(&file_name) {
                        return Some(entry.path().to_string_lossy().into_owned());
                    }
                }
            }
        }

        // Move to next directory based on direction
        match direction {
            SearchDirection::Parent => {
                // Try to get parent directory
                if let Some(parent) = Path::new(&current_dir).parent() {
                    if let Some(parent_str) = parent.to_str() {
                        current_dir = parent_str.to_string();
                    } else {
                        break; // Invalid UTF-8 in path
                    }
                } else {
                    break; // No more parent directories
                }
            }
            SearchDirection::Child => {
                // Get all subdirectories in current directory
                if let Ok(entries) = fs::read_dir(&current_dir) {
                    for entry in entries.flatten() {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_dir() {
                                if let Some(path_str) = entry.path().to_str() {
                                    // Recursively check this subdirectory
                                    if let Some(found_path) = recursively_check_for_file(
                                        path_str,
                                        file_pattern,
                                        levels - 1,
                                        SearchDirection::Child,
                                    ) {
                                        return Some(found_path);
                                    }
                                }
                            }
                        }
                    }
                }
                break; // Done checking current level's subdirectories
            }
        }
    }

    None
}
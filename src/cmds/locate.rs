use crate::cmds::build;
use crate::utils;
use clipboard::{ClipboardContext, ClipboardProvider};
use path_absolutize::Absolutize;
use std::path::Path;

/// Prints out the full path to the project DLL
pub fn execute(starting_dir: &str) {
    // First: build the project:
    match build::build_csharp_project(starting_dir) {
        Ok(_) => (),
        Err(e) => println!("Could not build project: {}", e),
    }
    match get_main_dll_path(true, starting_dir) {
        Ok(path) => {
            println!("{}", path);
            // Attempt to copy the path to the clipboard:
            if let Ok(mut ctx) = ClipboardContext::new() {
                _ = ctx.set_contents(path.to_owned());
            }
        }
        Err(e) => println!("Could not find DLL for project: {}", e),
    }
}

fn get_csproj_path(starting_dir: &str) -> Option<String> {
    utils::recursively_check_for_file(starting_dir, "*.csproj", 3, utils::SearchDirection::Child)
}

#[derive(Clone)]
pub struct ProjectInfo {
    /// The name of the project + .csproj
    pub full_project_name: String,
    pub project_name: String,
    /// As you can guess: the directory that the project csproj file resides in
    pub csproj_dir: String,
}
pub fn get_project_info(starting_dir: &str) -> Result<ProjectInfo, String> {
    let csproj_path = get_csproj_path(starting_dir);
    if let Some(csproj_path) = csproj_path {
        let csproj_dir = Path::new(&csproj_path)
            .parent()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let full_project_name = Path::new(&csproj_path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let project_name = full_project_name.replace(".csproj", "");
        return Ok(ProjectInfo {
            project_name,
            full_project_name,
            csproj_dir,
        });
    }
    Err("No csproj file found".to_string())
}

pub fn get_main_dll_path(absolute: bool, starting_dir: &str) -> Result<String, String> {
    let proj_info = get_project_info(starting_dir);
    if let Ok(proj_info) = proj_info {
        let name = proj_info.project_name;
        get_project_dll_path(absolute, name, starting_dir)
    } else {
        Err("No csproj file found".to_string())
    }
}

/// Returns the path to the project's DLL file or an error message if it cannot be found.
///
/// If absolute is true, the path will be absolute.
/// Otherwise, the path will be relative to the csproj file.
///
/// Do not include the .dll extension in the name.
pub fn get_project_dll_path(
    absolute: bool,
    name: String,
    starting_dir: &str,
) -> Result<String, String> {
    let proj_info = get_project_info(starting_dir);
    if let Ok(proj_info) = proj_info {
        let csproj_dir = proj_info.csproj_dir;

        let dll_name = name + ".dll";
        let dll_path = utils::recursively_check_for_file(
            &csproj_dir,
            &dll_name,
            3,
            utils::SearchDirection::Child,
        );
        if let Some(dll_path) = dll_path {
            if absolute {
                // Get absolute path without the \\?\ prefix
                match Path::new(&dll_path).absolutize() {
                    Ok(abs_path) => Ok(abs_path.to_string_lossy().into_owned()),
                    Err(_) => Ok(dll_path),
                }
            } else {
                // Convert absolute path to relative path from the csproj directory
                if let Ok(absolute_path) = Path::new(&dll_path).absolutize() {
                    if let Ok(csproj_absolute) = Path::new(&csproj_dir).absolutize() {
                        if let Some(relative_path) =
                            pathdiff::diff_paths(absolute_path, csproj_absolute)
                        {
                            return Ok(relative_path.to_string_lossy().into_owned());
                        }
                    }
                }
                // Fall back to original path if conversion fails
                Ok(dll_path)
            }
        } else {
            Err(format!(
                "Could not find dll for project. Expected to find {}",
                dll_name
            ))
        }
    } else {
        Err("Could not find csproj file".to_string())
    }
}

/// Returns a vector of all absolute DLL paths for the project
///
/// This function will find all .dll files in the project directory and its subdirectories
/// up to 3 levels deep, and return their absolute paths.
pub fn get_all_project_dll_paths(starting_dir: &str) -> Result<Vec<String>, String> {
    let proj_info = get_project_info(starting_dir)?;
    let csproj_dir = proj_info.csproj_dir;

    let mut absolute_dll_paths = Vec::new();

    // Walk through the project directory and find all .dll files
    if let Ok(entries) = std::fs::read_dir(&csproj_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if file_name.ends_with(".dll") {
                            // Convert to absolute path
                            match entry.path().absolutize() {
                                Ok(abs_path) => {
                                    absolute_dll_paths
                                        .push(abs_path.to_string_lossy().into_owned());
                                }
                                Err(_) => {
                                    // Fall back to original path if absolutization fails
                                    absolute_dll_paths
                                        .push(entry.path().to_string_lossy().into_owned());
                                }
                            }
                        }
                    }
                } else if file_type.is_dir() {
                    // Recursively check subdirectories (up to 2 more levels)
                    if let Some(path_str) = entry.path().to_str() {
                        if let Ok(sub_dlls) = get_all_dlls_in_directory(path_str, 2) {
                            absolute_dll_paths.extend(sub_dlls);
                        }
                    }
                }
            }
        }
    }

    Ok(absolute_dll_paths)
}

/// Helper function to recursively find all DLL files in a directory
fn get_all_dlls_in_directory(
    directory: &str,
    remaining_levels: usize,
) -> Result<Vec<String>, String> {
    let mut dll_paths = Vec::new();

    if remaining_levels == 0 {
        return Ok(dll_paths);
    }

    if let Ok(entries) = std::fs::read_dir(directory) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if file_name.ends_with(".dll") {
                            // Convert to absolute path
                            match entry.path().absolutize() {
                                Ok(abs_path) => {
                                    dll_paths.push(abs_path.to_string_lossy().into_owned());
                                }
                                Err(_) => {
                                    // Fall back to original path if absolutization fails
                                    dll_paths.push(entry.path().to_string_lossy().into_owned());
                                }
                            }
                        }
                    }
                } else if file_type.is_dir() {
                    // Recursively check subdirectories
                    if let Some(path_str) = entry.path().to_str() {
                        if let Ok(sub_dlls) =
                            get_all_dlls_in_directory(path_str, remaining_levels - 1)
                        {
                            dll_paths.extend(sub_dlls);
                        }
                    }
                }
            }
        }
    }

    Ok(dll_paths)
}

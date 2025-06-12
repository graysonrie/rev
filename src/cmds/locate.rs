use crate::cmds::build;
use crate::utils;
use clipboard::{ClipboardContext, ClipboardProvider};
use path_absolutize::Absolutize;
use std::path::Path;

/// Prints out the full path to the project DLL
pub fn execute() {
    // First: build the project:
    match build::build_csharp_project() {
        Ok(_) => (),
        Err(e) => println!("Could not build project: {}", e),
    }
    match get_main_dll_path(true) {
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

fn get_csproj_path() -> Option<String> {
    utils::recursively_check_for_file(".", "*.csproj", 3, utils::SearchDirection::Child)
}

pub struct ProjectInfo {
    /// The name of the project + .csproj
    pub full_project_name: String,
    pub project_name: String,
    /// As you can guess: the directory that the project csproj file resides in
    pub csproj_dir: String,
}
pub fn get_project_info() -> Result<ProjectInfo, String> {
    let csproj_path = get_csproj_path();
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

pub fn get_main_dll_path(absolute: bool) -> Result<String, String> {
    let proj_info = get_project_info();
    if let Ok(proj_info) = proj_info {
        let name = proj_info.project_name;
        get_project_dll_path(absolute, name)
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
pub fn get_project_dll_path(absolute: bool, name: String) -> Result<String, String> {
    let proj_info = get_project_info();
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

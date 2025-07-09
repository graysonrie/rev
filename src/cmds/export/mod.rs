pub mod addin_file;
pub mod web_app;
use crate::cmds::build;
use crate::cmds::locate;
use crate::utils::error_list::ErrorList;
use std::path::Path;
use std::path::PathBuf;

/// Exports the addin to the Revit addins directory, printing all errors and warnings to the console
///
/// This function will:
/// - Build the project
/// - Copy the DLLs to the addin directory
/// - Copy the .addin file to the Revit addins directory
/// - Print out the path to the addin
pub fn execute_auto(starting_dir: &str, for_version: &str, extra_dlls: &[&str]) {
    let destination_dir = get_revit_addins_path(for_version).unwrap();
    let errors = execute(starting_dir, extra_dlls, &destination_dir);
    if errors.has_errors() {
        println!(
            "Build failed with {} errors and {} warnings",
            errors.view_errors().len(),
            errors.view_warnings().len()
        );
    }
    for error in errors.view_errors() {
        println!("Error: {}", error);
    }
    for warning in errors.view_warnings() {
        println!("Warning: {}", warning);
    }
}

pub fn execute(starting_dir: &str, extra_dlls: &[&str], destination_dir: &Path) -> ErrorList {
    let mut dlls_to_export = Vec::new();
    let project_info = locate::get_project_info(starting_dir);
    let mut error_list = ErrorList::new();
    if let Ok(ref project_info) = project_info {
        dlls_to_export.push(project_info.project_name.clone() + ".dll");
    } else {
        error_list.add_error(
            "Error getting project info. Ensure you have a .csproj file in the current directory.",
        );
        return error_list;
    }
    match locate::get_main_dll_path(true, starting_dir) {
        Ok(dll_path) => {
            dlls_to_export.push(dll_path);
        }
        Err(e) => {
            error_list.add_error(&format!("Error getting project DLL path: {}", e));
            return error_list;
        }
    }
    for dll in extra_dlls.iter() {
        match locate::get_project_dll_path(true, dll.to_string(), starting_dir) {
            Ok(dll_path) => {
                dlls_to_export.push(dll_path);
            }
            Err(e) => {
                error_list.add_warning(&format!(
                    "Warning: could not find DLL for {}: {}",
                    dll.clone(),
                    e
                ));
            }
        }
    }

    match addin_file::handle_addin_file(starting_dir) {
        Ok(addin_file_path) => {
            if let Err(e) = build::build_csharp_project(starting_dir) {
                error_list.add_error(&format!("Error building project: {}", e));
            }
            // Clone values to avoid moving them in the loop
            let project_info_clone = project_info.clone();
            let dlls_to_export_clone = dlls_to_export.clone();

            // Create a new directory for the addin
            let addin_name = project_info_clone.as_ref().unwrap().project_name.clone();
            let addin_dir = destination_dir.join(addin_name.clone());
            std::fs::create_dir_all(&addin_dir).unwrap();

            // Copy all DLLs to the addin directory
            for dll_path in &dlls_to_export_clone {
                let dll_path = Path::new(&dll_path);
                if dll_path.exists() {
                    let local_dll_path = dll_path.file_name().unwrap();
                    let target_path = addin_dir.join(local_dll_path);
                    if let Err(e) = std::fs::copy(dll_path, target_path) {
                        error_list.add_error(&format!(
                            "Error copying DLL {}: {}",
                            local_dll_path.to_string_lossy(),
                            e
                        ));
                    } else {
                        error_list.add_warning(&format!(
                            "Successfully copied DLL: {}",
                            local_dll_path.to_string_lossy()
                        ));
                    }
                } else {
                    error_list.add_warning(&format!(
                        "Warning: DLL not found at path: {}",
                        dll_path.to_string_lossy()
                    ));
                }
            }

            // Copy the .addin file to the Revit addins directory
            let target_addin_file_path = destination_dir.join(format!("{}.addin", addin_name));
            if let Err(e) = std::fs::copy(&addin_file_path, target_addin_file_path) {
                error_list.add_error(&format!("Error copying .addin file: {}", e));
            } else {
                error_list.add_warning("Successfully copied .addin file");
            }

            error_list.add_warning("Addin exported successfully");
        }
        Err(e) => {
            error_list.add_error(&format!("Error creating addin file: {}", e));
        }
    }
    error_list
}

/// Returns the path to the Revit addin folder or an error message if it cannot be found.
///
/// Version should be the year of the Revit version, e.g. "2025"
pub fn get_revit_addins_path(version: &str) -> Result<PathBuf, String> {
    let appdata_roaming = dirs::data_dir().unwrap();
    let directory = appdata_roaming
        .join("Autodesk\\Revit\\Addins")
        .join(version);
    if directory.exists() {
        Ok(directory)
    } else {
        Err(format!(
            "Expected to find addin folder in {}",
            directory.to_string_lossy().into_owned()
        ))
    }
}

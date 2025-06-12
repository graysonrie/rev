pub mod addin_file;
pub mod web_app;
use crate::cmds::build;
use crate::cmds::locate;
use crate::ensure_revit_version_is_set;
use std::path::Path;
use std::path::PathBuf;

pub fn execute() {
    ensure_revit_version_is_set();
    let mut dlls_to_export = Vec::new();
    let project_info = locate::get_project_info();
    if let Ok(ref project_info) = project_info {
        dlls_to_export.push(project_info.project_name.clone() + ".dll");
    } else {
        println!(
            "Error getting project info. Ensure you have a .csproj file in the current directory."
        );
        return;
    }
    match locate::get_main_dll_path(true) {
        Ok(dll_path) => {
            dlls_to_export.push(dll_path);
        }
        Err(e) => {
            println!("Error getting project DLL path: {}", e);
            return;
        }
    }
    let other_dlls = ["RealRevitPlugin".to_string()];
    for dll in other_dlls.iter() {
        match locate::get_project_dll_path(true, dll.clone()) {
            Ok(dll_path) => {
                dlls_to_export.push(dll_path);
            }
            Err(e) => {
                println!("Warning: could not find DLL for {}: {}", dll.clone(), e);
            }
        }
    }

    match addin_file::handle_addin_file() {
        Ok(addin_file_path) => {
            if let Err(e) = build::build_csharp_project() {
                println!("Error building project: {}", e);
            }
            match get_revit_addins_path() {
                Ok(revit_addins_path) => {
                    // Create a new directory for the addin
                    let addin_name = project_info.unwrap().project_name;
                    let addin_dir = revit_addins_path.join(addin_name.clone());
                    std::fs::create_dir_all(&addin_dir).unwrap();

                    // Copy all DLLs to the addin directory
                    for dll_path in dlls_to_export {
                        let dll_path = Path::new(&dll_path);
                        if dll_path.exists() {
                            let local_dll_path = dll_path.file_name().unwrap();
                            let target_path = addin_dir.join(local_dll_path);
                            if let Err(_e) = std::fs::copy(dll_path, target_path) {
                                // println!(
                                //     "Error copying DLL {}: {}",
                                //     local_dll_path.to_string_lossy(),
                                //     e
                                // );
                            } else {
                                // println!(
                                //     "Successfully copied DLL: {}",
                                //     local_dll_path.to_string_lossy()
                                // );
                            }
                        } else {
                            // println!(
                            //     "Warning: DLL not found at path: {}",
                            //     dll_path.to_string_lossy()
                            // );
                        }
                    }

                    // Copy the .addin file to the Revit addins directory
                    let target_addin_file_path =
                        revit_addins_path.join(format!("{}.addin", addin_name));
                    if let Err(e) = std::fs::copy(&addin_file_path, target_addin_file_path) {
                        println!("Error copying .addin file: {}", e);
                    } else {
                        println!("Successfully copied .addin file");
                    }

                    println!("Addin exported successfully");
                }
                Err(e) => {
                    println!("Error getting revit addin path: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Error creating addin file: {}", e);
        }
    }
}

/// Returns the path to the Revit addin folder or an error message if it cannot be found.
pub fn get_revit_addins_path() -> Result<PathBuf, String> {
    let revit_version = ensure_revit_version_is_set();
    let appdata_roaming = dirs::data_dir().unwrap();
    let directory = appdata_roaming
        .join("Autodesk\\Revit\\Addins")
        .join(revit_version);
    if directory.exists() {
        Ok(directory)
    } else {
        Err(format!(
            "Expected to find addin folder in {}",
            directory.to_string_lossy().into_owned()
        ))
    }
}

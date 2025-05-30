pub mod addin_file;
pub mod web_app;
use crate::cmds::build;
use crate::cmds::locate;
use crate::ensure_revit_version_is_set;
use std::path::Path;
use std::path::PathBuf;

pub fn execute() {
    ensure_revit_version_is_set();
    match addin_file::handle_addin_file() {
        Ok(addin_file_path) => {
            if let Err(e) = build::build_csharp_project() {
                println!("Error building project: {}", e);
            }
            match locate::get_project_dll_path(true) {
                Ok(dll_path) => match get_revit_addins_path() {
                    Ok(revit_addins_path) => {
                        let dll_path = Path::new(&dll_path); // This will point to the DLL file inside bin/Debug
                        let local_dll_path = dll_path.file_name().unwrap();
                        let addin_name = local_dll_path
                            .to_string_lossy()
                            .into_owned()
                            .replace(".dll", "");
                        // Create a new directory for the addin. This will contain the DLL of the addin.
                        let addin_dir = revit_addins_path.join(&addin_name);
                        std::fs::create_dir_all(&addin_dir).unwrap();

                        // Copy the DLL to the addin directory:
                        let target_path = addin_dir.join(local_dll_path);
                        std::fs::copy(dll_path, target_path).unwrap();
                        // Copy the .addin file to the Revit addins directory:
                        let target_addin_file_path =
                            revit_addins_path.join(format!("{}.addin", addin_name));
                        std::fs::copy(&addin_file_path, target_addin_file_path).unwrap();

                        // Try to build the web app if it exists:
                        web_app::build_if_exists(&addin_dir);

                        println!("Addin exported successfully");
                    }
                    Err(e) => {
                        println!("Error getting revit addin path: {}", e);
                    }
                },
                Err(e) => {
                    println!("Error getting project DLL path: {}", e);
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

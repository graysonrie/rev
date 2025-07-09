mod cmds;
mod external_cmds;
mod state;
mod utils;

use std::fmt::Display;
use std::path::Path;

pub use cmds::build;

use crate::cmds::{export, locate};
pub use crate::utils::error_list::ErrorList;

/// Builds the project in the given directory. Returns the output from the build command if it was successful, or an error message.
pub fn build_project(starting_dir: &str) -> Result<String, String> {
    build::build_csharp_project(starting_dir)
}
/// Exports the addin to the given destination directories. Returns an error list if any errors occur.
///
/// `starting_dir` is the directory that contains the C# project.
/// `extra_dlls` are any additional DLLs that need to be exported.
/// `destination_directories` are the directories to export the addin to.
pub fn export_addin_multiple(
    starting_dir: &str,
    extra_dlls: &[&str],
    destination_directories: &[&Path],
) -> ErrorList {
    if let Err(e) = build_project(starting_dir) {
        let mut error_list = ErrorList::new();
        error_list.add_error(&e);
        return error_list;
    }
    let mut error_list = ErrorList::new();
    for destination_dir in destination_directories {
        error_list.extend(&export::execute(starting_dir, extra_dlls, destination_dir));
    }
    error_list
}

/// Builds the addin, then exports the addin to the given destination directory. Returns an error list if any errors occur.
///
/// `starting_dir` is the directory that contains the C# project.
/// `extra_dlls` are any additional DLLs that need to be exported.
/// `destination_dir` is the directory to export the addin to.
pub fn export_addin(starting_dir: &str, extra_dlls: &[&str], destination_dir: &Path) -> ErrorList {
    if let Err(e) = build_project(starting_dir) {
        let mut error_list = ErrorList::new();
        error_list.add_error(&e);
        return error_list;
    }
    export::execute(starting_dir, extra_dlls, destination_dir)
}

#[derive(Debug, Clone)]
pub enum GetAddinFileInfoError {
    FileNotFound,
    AddinFileError(export::addin_file::GetAddinFileInfoError),
}
impl Display for GetAddinFileInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Gets the addin file info from the given path. Returns an error if the file is not found or if the file is not a valid addin file. Otherwise, returns the contents of the '.addin' file.
///
/// `starting_dir` is the directory that contains the C# project. This is the directory that contains the '.addin' file as well.
pub fn get_addin_file_info(
    starting_dir: &str,
) -> Result<export::addin_file::AddinFileInfo, GetAddinFileInfoError> {
    let project_name =
        get_project_name(starting_dir).map_err(|_| GetAddinFileInfoError::FileNotFound)?;
    let parent_dir = Path::new(starting_dir);

    let addin_file_path = parent_dir.join(format!("{}.addin", project_name));
    export::addin_file::get_addin_file_info(&addin_file_path.to_string_lossy())
        .map_err(GetAddinFileInfoError::AddinFileError)
}

/// Gets the addin file info from the given path. Returns an error if the file is not found or if the file is not a valid addin file. Otherwise, returns the contents of the '.addin' file.
///
/// `addin_file_path` is the path to the '.addin' file.
pub fn get_addin_file_info_from_file(
    addin_file_path: &str,
) -> Result<export::addin_file::AddinFileInfo, GetAddinFileInfoError> {
    export::addin_file::get_addin_file_info(addin_file_path)
        .map_err(GetAddinFileInfoError::AddinFileError)
}

#[derive(Debug, Clone)]
pub enum CreateAddinFileError {
    FileNotFound,
    AddinFileError(String),
}
impl Display for CreateAddinFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Creates an addin file for the project. Returns an error if the file is not found or if the file is not a valid addin file. Will not overwrite an existing addin file.
///
/// `starting_dir` is the directory that contains the C# project. This is the directory that contains the '.addin' file as well.
/// `addin_info` is the information to write to the '.addin' file.
///
/// Returns the path to the addin file if it was created successfully, or an error if the file is not found or if the file is not a valid addin file.
pub fn create_addin_file_for_project(
    starting_dir: &str,
    addin_info: export::addin_file::AddinFileInfo,
) -> Result<String, CreateAddinFileError> {
    let project_name =
        get_project_name(starting_dir).map_err(|_| CreateAddinFileError::FileNotFound)?;
    // The starting directory is the directory that contains the C# project. It is also the parent in this scenario
    let parent_dir = Path::new(starting_dir);

    let addin_file_path = parent_dir.join(format!("{}.addin", project_name));
    export::addin_file::create_addin_file(&addin_file_path, addin_info)
        .map_err(|e| CreateAddinFileError::AddinFileError(e.to_string()))?;
    Ok(addin_file_path.to_string_lossy().to_string())
}

/// Gets all the DLLs in the project. Returns an error if no DLLs are found.
///
/// `starting_dir` is the directory that contains the C# project.
pub fn get_project_dlls(starting_dir: &str) -> Result<Vec<String>, String> {
    locate::get_all_project_dll_paths(starting_dir)
}

/// Gets the name of the project. Returns an error if no project name is found.
///
/// `starting_dir` is the directory that contains the C# project.
pub fn get_project_name(starting_dir: &str) -> Result<String, String> {
    let csproj_path = utils::recursively_check_for_file(
        starting_dir,
        "*.csproj",
        3,
        utils::SearchDirection::Child,
    );
    if let Some(csproj_path) = csproj_path {
        let project_name = Path::new(&csproj_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".csproj")
            .unwrap();
        Ok(project_name.to_string())
    } else {
        Err("No csproj file found".to_string())
    }
}

pub use cmds::export::addin_file::AddinFileInfo;

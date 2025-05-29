use std::path::Path;
use std::process::Command;

const VS_MSBUILD_PATH: &str =
    r"C:\Program Files\Microsoft Visual Studio\2022\Community\MSBuild\Current\Bin\MSBuild.exe";

pub enum MsBuildError {
    NotFound,
    Output(String),
}
pub fn build_project(project_path: &str) -> Result<String, MsBuildError> {
    // Try system-wide msbuild first
    let result = Command::new("msbuild").arg(project_path).output();

    match result {
        Ok(output) => {
            if !output.status.success() {
                return Err(MsBuildError::Output(
                    String::from_utf8_lossy(&output.stderr).to_string(),
                ));
            }
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
        Err(_) => {
            // If system-wide fails, try Visual Studio path
            println!("System-wide MSBuild not found, trying Visual Studio path...");
            if Path::new(VS_MSBUILD_PATH).exists() {
                match Command::new(VS_MSBUILD_PATH).arg(project_path).output() {
                     Ok(output) => {
                        if !output.status.success() {
                            return Err(MsBuildError::Output(
                                String::from_utf8_lossy(&output.stderr).to_string(),
                            ));
                        }
                        Ok(String::from_utf8_lossy(&output.stdout).to_string())
                    }
                    Err(e) => Err(MsBuildError::Output(format!(
                        "Failed to run MSBuild from Visual Studio path: {}",
                        e
                    ))),
                }
            } else {
                Err(MsBuildError::NotFound)
            }
        }
    }
}

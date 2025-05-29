use std::process::Command;

pub fn check_if_exists() -> bool {
    let result = Command::new("dotnet").arg("--version").output();
    result.is_ok()
}

pub enum DotnetError {
    NotFound,
    Output(String),
}
pub fn build_project(project_path: &str) -> Result<String, DotnetError> {
    if !check_if_exists() {
        return Err(DotnetError::NotFound);
    }
    let result = Command::new("dotnet")
        .arg("build")
        .arg(project_path)
        .output();
    match result {
        Ok(output) => {
            if !output.status.success() {
                return Err(DotnetError::Output(
                    String::from_utf8_lossy(&output.stderr).to_string(),
                ));
            }
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
        Err(e) => Err(DotnetError::Output(format!(
            "Failed to run dotnet build: {}",
            e
        ))),
    }
}

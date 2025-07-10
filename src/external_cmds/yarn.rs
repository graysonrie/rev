use std::os::windows::process::CommandExt;
use std::path::Path;
use tokio::process::Command;

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn get_yarn_path() -> Option<String> {
    // Common paths where yarn might be installed
    let user_profile = std::env::var("USERPROFILE").unwrap_or_default();
    let user_yarn_path = format!("{}\\AppData\\Roaming\\npm\\yarn.cmd", user_profile);

    // println!("Looking for yarn in common installation locations...");

    let possible_paths = vec![
        // npm global installation path
        r"C:\Program Files\nodejs\yarn.cmd",
        r"C:\Program Files (x86)\nodejs\yarn.cmd",
        // User's AppData npm path
        &user_yarn_path,
        // Try yarn from PATH as last resort
        "yarn",
    ];

    for path in possible_paths {
        // println!("Checking path: {}", path);
        if path == "yarn" {
            // For the PATH-based check, use the --version command
            // Note: This is still synchronous since we're in a sync function
            if std::process::Command::new(path)
                .arg("--version")
                .creation_flags(CREATE_NO_WINDOW)
                .output()
                .is_ok()
            {
                // println!("Found yarn in PATH");
                return Some(path.to_string());
            }
        } else if Path::new(path).exists() {
            // println!("Found yarn at: {}", path);
            return Some(path.to_string());
        }
    }
    // println!("Could not find yarn in any common locations");
    None
}

pub enum YarnError {
    NotFound,
    Output(String),
}

pub async fn build(working_dir: &str) -> Result<String, YarnError> {
    let yarn_path = match get_yarn_path() {
        Some(path) => path,
        None => return Err(YarnError::NotFound),
    };

    println!("Using yarn from: {}", yarn_path);
    println!("Running in directory: {}", working_dir);

    let result = Command::new(yarn_path)
        .arg("build")
        .current_dir(working_dir)
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .await;

    match result {
        Ok(output) => {
            if !output.status.success() {
                return Err(YarnError::Output(
                    String::from_utf8_lossy(&output.stderr).to_string(),
                ));
            }
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
        Err(e) => Err(YarnError::Output(format!(
            "Failed to run yarn build: {}",
            e
        ))),
    }
}

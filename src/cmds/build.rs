use crate::{external_cmds::{dotnet::{self, DotnetError}, msbuild::{self, MsBuildError}}, utils};


pub fn execute(starting_dir: &str) {
    match build_csharp_project(starting_dir) {
        Ok(_output) => {
            println!("Project successfully built");
        }
        Err(msg) => {
            println!("Build Error: {}", msg)
        }
    }
}

/// Returns the output from the build command if it was successful, or an error message.
pub fn build_csharp_project(starting_dir: &str) -> Result<String, String> {
    let csproj_path =
        utils::recursively_check_for_file(starting_dir, "*.csproj", 3, utils::SearchDirection::Child);
    if let Some(csproj_path) = csproj_path {
        match msbuild::build_project(&csproj_path) {
            Ok(output) => Ok(output),
            Err(e) => {
                match e {
                    MsBuildError::NotFound => {
                        println!(
                            "Could not find MSBuild installation. Defaulting to using dotnet to build the project..."
                        );
                    }
                    MsBuildError::Output(output) => {
                        println!("Error building with MSBuild: {}", output);
                    }
                }
                // Try dotnet since msbuild did not work:
                match dotnet::build_project(&csproj_path) {
                    Ok(output) => Ok(output),
                    Err(dotnet_err) => match dotnet_err {
                        DotnetError::NotFound => {
                            Err("Could not find Dotnet installation on system".to_string())
                        }
                        DotnetError::Output(output) => {
                            Err(format!("Error building with dotnet: {}", output))
                        }
                    },
                }
            }
        }
    } else {
        Err("No csproj file found".to_string())
    }
}


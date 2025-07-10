use std::path::{Path, PathBuf};

use path_absolutize::Absolutize;

use crate::external_cmds::yarn;
use crate::utils;

// Checks for the presence of a web app (Right now we should only support Next.js + yarn) and will generate
// the static files for the app, rename the bundle to 'web' and then bundle that folder with the assets of the
// Revit add in

/// Builds the web project if it exists. Logs any errors
///
/// `addin_dir` should be the directory that contains the DLL file for your addin,
/// not the outer Revit addins directory
pub async fn build_if_exists(addin_dir: &Path) {
    if let Some(node_modules) = find_node_modules_path(".") {
        println!("Exporting static assets for web app. Please wait...");
        match create_static_export(&node_modules).await {
            Ok(_) => {
                let node_modules_path = Path::new(&node_modules);
                let parent = node_modules_path.parent().unwrap();
                let out_dir = parent.join("out");

                if !Path::exists(&out_dir) {
                    println!(
                        "Web app was exported, and expected to find static files at {}, but they were not found",
                        out_dir.to_string_lossy()
                    );
                    return;
                }
                let new_out_path = addin_dir.join("web");
                std::fs::create_dir_all(new_out_path.clone()).unwrap();
                println!("{}", out_dir.to_string_lossy());
                println!("{}", new_out_path.to_string_lossy());
                std::fs::copy(out_dir, new_out_path).unwrap();
            }
            Err(err) => {
                println!("Error exporting web app: {}", err)
            }
        }
    } else {
        println!("No web app found");
    }
}

/// Returns the absolute path to the node modules directory
fn find_node_modules_path(starting_dir: &str) -> Option<PathBuf> {
    utils::recursively_check_for_file(
        starting_dir,
        "node_modules",
        3,
        utils::SearchDirection::Child,
    )
    .map(|x| {
        let path = Path::new(&x);
        let absolute = path.absolutize().unwrap();
        absolute.into_owned()
    })
}

async fn create_static_export(node_modules_path: &Path) -> Result<(), String> {
    let parent = node_modules_path.parent().unwrap();

    // Run yarn build in the parent directory
    match yarn::build(parent.to_str().unwrap()).await {
        Ok(_) => Ok(()),
        Err(yarn::YarnError::NotFound) => Err("Yarn is not installed on this system".to_string()),
        Err(yarn::YarnError::Output(e)) => Err(format!("Failed to run yarn build: {}", e)),
    }
}

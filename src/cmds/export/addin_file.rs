use crate::state;
use crate::utils;
use crate::utils::input::{prompt_user, prompt_user_with_default};
use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    path::Path,
};

pub fn handle_addin_file() -> Result<String, String> {
    let csproj_path =
        utils::recursively_check_for_file(".", "*.csproj", 3, utils::SearchDirection::Child);
    if let Some(csproj_path) = csproj_path {
        let parent_dir = Path::new(&csproj_path).parent().unwrap();
        let project_name = Path::new(&csproj_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let project_name = project_name.strip_suffix(".csproj").unwrap();

        let addin_file_path = parent_dir.join(format!("{}.addin", project_name));
        if is_addin_file_a_template_or_missing(&addin_file_path) {
            let addin_info = prompt_user_for_addin_file_info(project_name);
            create_addin_file(&addin_file_path, addin_info).map_err(|e| e.to_string())?;
        }
        Ok(addin_file_path.to_string_lossy().into_owned())
    } else {
        Err("No csproj file found".to_string())
    }
}

/// Returns true if the addin file contains template information or does not exist
fn is_addin_file_a_template_or_missing(path: &Path) -> bool {
    let mut addin_file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return true,
    };
    let mut contents = String::new();
    addin_file.read_to_string(&mut contents).unwrap();

    contents.contains("Template Plugin")
        || contents.contains("youremail@example.com")
        || contents.contains("Insert description here")
}

fn prompt_user_for_addin_file_info(project_name: &str) -> AddinFileInfo {
    let state = state::get_state_or_default();

    let name = prompt_user("Enter the name of your addin");
    let assembly = format!("{}\\{}.dll", project_name, project_name);
    let addin_id = uuid::Uuid::new_v4().to_string();
    let full_class_name = format!("{}.App", project_name);
    let vendor_id = prompt_user_with_default("Enter your vendor ID", "Development");
    let vendor_description = prompt_user("Enter a description of your add-in");

    let vendor_email = if state.email_address.is_empty() {
        prompt_user("Enter your work email address")
    } else {
        prompt_user_with_default("Enter your work email address", &state.email_address)
    };

    state::save_state(&state::State {
        email_address: vendor_email.clone(),
        ..state
    });

    AddinFileInfo {
        name,
        assembly,
        addin_id,
        full_class_name,
        vendor_id,
        vendor_description,
        vendor_email,
    }
}

struct AddinFileInfo {
    pub name: String,
    pub assembly: String,
    pub addin_id: String,
    pub full_class_name: String,
    pub vendor_id: String,
    pub vendor_description: String,
    pub vendor_email: String,
}

fn create_addin_file(path: &Path, addin_info: AddinFileInfo) -> Result<(), std::io::Error> {
    let addin_file = File::create(path)?;
    let mut addin_file = BufWriter::new(addin_file);

    writeln!(addin_file, "<?xml version=\"1.0\" encoding=\"utf-8\"?>")?;
    writeln!(addin_file, "<RevitAddIns>")?;
    writeln!(addin_file, "\t<AddIn Type=\"Application\">")?;
    writeln!(addin_file, "\t\t<Name>{}</Name>", addin_info.name)?;
    writeln!(
        addin_file,
        "\t\t<Assembly>{}</Assembly>",
        addin_info.assembly
    )?;
    writeln!(addin_file, "\t\t<AddInId>{}</AddInId>", addin_info.addin_id)?;
    writeln!(
        addin_file,
        "\t\t<FullClassName>{}</FullClassName>",
        addin_info.full_class_name
    )?;
    writeln!(
        addin_file,
        "\t\t<VendorId>{}</VendorId>",
        addin_info.vendor_id
    )?;
    writeln!(
        addin_file,
        "\t\t<VendorDescription>{}</VendorDescription>",
        addin_info.vendor_description
    )?;
    writeln!(
        addin_file,
        "\t\t<VendorEmail>{}</VendorEmail>",
        addin_info.vendor_email
    )?;
    writeln!(addin_file, "\t</AddIn>")?;
    writeln!(addin_file, "</RevitAddIns>")?;

    Ok(())
}

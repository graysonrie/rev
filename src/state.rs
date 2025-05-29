use std::fs::File;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct State {
    pub revit_version: String,
    pub email_address: String,
}

pub fn save_state(state: &State) {
    let app_data_dir = dirs::data_dir();

    if let Some(app_data_dir) = app_data_dir {
        let state_file_path = app_data_dir.join("rev");
        if !state_file_path.exists() {
            std::fs::create_dir_all(state_file_path.clone()).unwrap();
        }
        let state_json_path = state_file_path.join("RevitState.json");
        if let Ok(state_file) = File::create(state_json_path) {
            serde_json::to_writer(state_file, state).unwrap();
        }
    }
}

pub fn get_state() -> Option<State> {
    let app_data_dir = dirs::data_dir();
    if let Some(app_data_dir) = app_data_dir {
        let state_file_path = app_data_dir.join("rev/RevitState.json");
        if let Ok(state_file) = File::open(state_file_path) {
            let state: State = serde_json::from_reader(state_file).unwrap();
            return Some(state);
        }
    }
    None
}

pub fn get_state_or_default() -> State {
    get_state().unwrap_or_default()
}

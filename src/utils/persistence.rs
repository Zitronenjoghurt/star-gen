use crate::utils::persistence::save_state::SaveState;
use bevy::log::error;
use bevy::prelude::World;
#[cfg(not(debug_assertions))]
use directories::ProjectDirs;
use std::path::PathBuf;

mod save_state;
mod settings;

#[cfg(not(debug_assertions))]
pub fn get_project_dirs() -> ProjectDirs {
    ProjectDirs::from("io.github", "zitronenjoghurt", "star-gen").unwrap()
}

pub fn get_save_dir() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        PathBuf::from("./")
    }
    #[cfg(not(debug_assertions))]
    {
        let project_dirs = crate::utils::persistence::get_project_dirs();
        project_dirs.data_dir().to_path_buf()
    }
}

pub fn get_save_file_path() -> PathBuf {
    get_save_dir().join("star-gen-save.json")
}

pub fn save_state(world: &World) {
    let directory = get_save_dir();
    if !directory.exists() {
        std::fs::create_dir_all(directory.clone()).unwrap();
    }

    let path = get_save_file_path();
    let state = SaveState::save(world);

    let Ok(data) = serde_json::to_string_pretty(&state) else {
        error!("Failed to serialize save state");
        return;
    };

    match std::fs::write(path, data) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to save state: {}", e);
        }
    }
}

pub fn save_state_exists() -> bool {
    let path = get_save_file_path();
    path.is_file()
}

pub fn load_state(world: &mut World) {
    let path = get_save_file_path();

    let Ok(data) = std::fs::read_to_string(path) else {
        error!("Failed to read save file");
        return;
    };

    let Ok(mut state) = serde_json::from_str::<SaveState>(&data) else {
        error!("Failed to deserialize save state");
        return;
    };

    state.apply_validations();
    state.load(world);
}

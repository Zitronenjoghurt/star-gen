use crate::events::star_cluster_generate::StarClusterGenerateEvent;
use crate::events::star_spawn::StarSpawnEvent;
use crate::persistence::save_state::SaveState;
use crate::persistence::star_cluster_state::StarClusterState;
use crate::types::cluster_generation_method::ClusterGenerationMethod;
use bevy::log::error;
use bevy::prelude::{EventWriter, Events, Mut, World};
#[cfg(not(debug_assertions))]
use directories::ProjectDirs;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Read, Write};
use std::path::PathBuf;

mod save_state;
mod settings;
mod star_cluster_state;
mod star_state;

#[cfg(not(debug_assertions))]
pub fn get_project_dirs() -> ProjectDirs {
    ProjectDirs::from("io.github", "zitronenjoghurt", "star-gen").unwrap()
}

pub fn get_save_dir() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        PathBuf::from("./debug_save")
    }
    #[cfg(not(debug_assertions))]
    {
        let project_dirs = crate::persistence::get_project_dirs();
        project_dirs.data_dir().to_path_buf()
    }
}

pub fn get_save_file_path() -> PathBuf {
    get_save_dir().join("star-gen-save.json")
}

pub fn get_last_cluster_path() -> PathBuf {
    get_save_dir().join("last-cluster.bin")
}

pub fn save_state(world: &World) {
    let directory = get_save_dir();
    if !directory.exists() {
        std::fs::create_dir_all(directory.clone()).unwrap();
    }

    let save_path = get_save_file_path();
    let save_state = SaveState::save(world);

    let Ok(data) = serde_json::to_string_pretty(&save_state) else {
        error!("Failed to serialize save state");
        return;
    };

    match std::fs::write(save_path, data) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to save state: {}", e);
        }
    }

    let cluster_path = get_last_cluster_path();
    let cluster_state = StarClusterState::save(world);
    let Ok(bytes) = bincode::serde::encode_to_vec(&cluster_state, bincode::config::standard())
    else {
        error!("Failed to serialize cluster state");
        return;
    };

    let mut compressor = ZlibEncoder::new(Vec::new(), Compression::best());
    compressor.write_all(&bytes).unwrap();
    let compressed_data = compressor.finish().unwrap();

    match std::fs::write(cluster_path, compressed_data) {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to save cluster state: {}", e);
        }
    }
}

pub fn save_state_exists() -> bool {
    let path = get_save_file_path();
    path.is_file()
}

pub fn load_state(world: &mut World) {
    let save_path = get_save_file_path();

    let Ok(data) = std::fs::read_to_string(save_path) else {
        error!("Failed to read save file");
        return;
    };

    let Ok(mut state) = serde_json::from_str::<SaveState>(&data) else {
        error!("Failed to deserialize save state");
        return;
    };

    state.apply_validations();
    state.load(world);

    let cluster_path = get_last_cluster_path();
    if cluster_path.exists() {
        let Ok(compressed_data) = std::fs::read(cluster_path) else {
            error!("Failed to read cluster state");
            return;
        };

        let mut decompressor = ZlibDecoder::new(compressed_data.as_slice());
        let mut decompressed_data = Vec::new();
        decompressor.read_to_end(&mut decompressed_data).unwrap();

        let cluster_state: StarClusterState = bincode::serde::decode_from_slice(
            decompressed_data.as_slice(),
            bincode::config::standard(),
        )
        .unwrap()
        .0;

        cluster_state.load(world);
    } else if let Some(mut event) = world.get_resource_mut::<Events<StarClusterGenerateEvent>>() {
        let method = ClusterGenerationMethod::cubic_with_seed(69);
        event.send(StarClusterGenerateEvent::new(method));
    }
}

pub fn delete_all_data() {
    let path = get_save_dir();
    if path.exists() {
        std::fs::remove_dir_all(path).unwrap();
    }
}

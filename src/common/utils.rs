use std::path::PathBuf;

use super::constants::SCRIPTS_FOLDER;

pub fn get_scripts_path() -> PathBuf {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    PathBuf::from(current_dir).join(SCRIPTS_FOLDER)
}

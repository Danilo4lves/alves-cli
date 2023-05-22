use std::{
    env::current_dir,
    fs::{create_dir, read_dir},
    process::Command,
};

use fs_extra::{copy_items, dir};

use super::logger::Logger;

fn clone_dev_config_repo() {
    // TODO: deal with absolute path
    Command::new("git")
        .arg("clone")
        .arg("git@github.com:Danilo4lves/dev-config.git")
        .output()
        .expect("There was an error while cloning dev config repo");
}

fn pull_dev_config_repo() {
    // TODO: deal with absolute path
    Command::new("git")
        .arg("pull")
        .output()
        .expect("There was an error while updating dev config repo");
}

fn fetch_dev_config() {
    Logger::info("Obtaining dev configuration...".to_string());

    let mut dev_config_dir = current_dir().expect("Error while getting current dir");
    dev_config_dir.push("dev-config");

    if dev_config_dir.exists() {
        pull_dev_config_repo();
    } else {
        clone_dev_config_repo();
    }
}

fn copy_configs() {
    let mut config_folder_path = home::home_dir().expect("Error while retrieving home directory");
    config_folder_path.push(".config");

    let config_folder_creation = create_dir(config_folder_path.clone());

    match config_folder_creation {
        Ok(_) => (),
        Err(err) => Logger::error(err.to_string()),
    }

    let options = dir::CopyOptions::new();
    let mut paths_to_copy = Vec::new();

    for entry in read_dir("./dev-config").unwrap() {
        let path = entry.unwrap().path();
        paths_to_copy.push(path);
    }

    let result = copy_items(&paths_to_copy, config_folder_path.clone(), &options);

    match result {
        Ok(_) => (),
        Err(err) => Logger::error(format!(
            "Error while copying dev config: {:?}",
            err.to_string()
        )),
    };
}

pub fn setup_dev_config() {
    fetch_dev_config();
    copy_configs();
}

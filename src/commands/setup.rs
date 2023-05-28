use crate::common::logger::Logger;

use super::{setup_dev_config, setup_neovim, setup_nvm, setup_zsh};

pub fn run(command: &Option<String>) {
    match command
        .clone()
        .expect("Setup command not provided")
        .as_str()
    {
        "nvm" => setup_nvm::run(),
        "dev-config" => setup_dev_config::run(),
        "neovim" => setup_neovim::run(),
        "zsh" => setup_zsh::run(),
        not_found => Logger::error(format!("Setup command not found: {}", not_found)),
    };
}

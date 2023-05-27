use std::process::Command;

use crate::common::{brew::install_brew, dev_config::setup_dev_config_repo, logger::Logger};

fn install_neovim() {
    Logger::info("Installing NeoVim...".to_string());

    let neovim_version = Command::new("neovim").arg("--version").output();

    match neovim_version {
        Ok(_) => {
            Logger::info("NeoVim is already installed. Skipping...".to_string());
            ()
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                let result = Command::new("brew")
                    .arg("install")
                    .arg("neovim")
                    .spawn()
                    .expect("Neovim to be installed")
                    .wait();

                match result {
                    Ok(_) => Logger::success("NeoVim was installed successfully".to_string()),
                    Err(err) => {
                        Logger::error(format!(
                            "There was a problem while installing NeoVim: {}",
                            err.to_string()
                        ));
                    }
                }
            }
            _ => Logger::error(err.to_string()),
        },
    };
}

pub fn run() {
    install_brew();
    install_neovim();
    setup_dev_config_repo();
}

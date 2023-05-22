use std::process::Command;

use super::logger::Logger;

pub fn install_brew() {
    let brew_version = Command::new("brew").arg("--version").output();

    match brew_version {
        Ok(_) => (),
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                Logger::info("Detected that brew is not installed. Installing...".to_string());

                Logger::warn(
                    "Brew installation is not working. Do it manually for the moment.".to_string(),
                );

                // TODO: this is not working
                let result = Command::new("/bin/bash")
                    .arg("-c")
                    .arg("\"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"")
                    .spawn()
                    .expect("Error while installing brew")
                    .wait();

                match result {
                    Ok(_) => Logger::info("Brew was installed successfully".to_string()),
                    Err(err) => {
                        Logger::error(format!(
                            "There was a problem while installing Brew: {}",
                            err.to_string()
                        ));
                    }
                }
            }
            _ => Logger::error(err.to_string()),
        },
    };
}

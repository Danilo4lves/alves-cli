use std::process::{Command, Stdio};

use crate::common::{constants::BASH_SHELL, logger::Logger, source_env::source_env};

pub fn run() {
    Logger::info("Installing NVM...".to_string());

    let output = Command::new("curl")
        .arg("-o-")
        .arg("https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to curl NVM installer");

    let stdout = output.stdout.expect("Failed to get NVM installer stdout");

    Command::new(BASH_SHELL)
        .stdin(Stdio::from(stdout))
        .spawn()
        .expect("Failed install NVM script")
        .wait()
        .expect("Failed to wait for NVM installer");

    source_env();

    Logger::success("NVM installed successfully!".to_string());
    Logger::info("Installing latest nodejs LTS...".to_string());

    Command::new("nvm")
        .arg("install")
        .arg("--lts")
        .envs(std::env::vars())
        .spawn()
        .expect("Failed install node LTS")
        .wait()
        .expect("Failed to wait for node LTS install");

    Logger::success("Latest nodejs LTS installed successfully!".to_string());
}

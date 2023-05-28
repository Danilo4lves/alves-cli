use crate::common::constants::BASH_SHELL;
use crate::common::cross_install::cross_install;
use crate::common::logger::Logger;
use std::process::Command;
use std::process::Stdio;

pub fn run() {
    Logger::info("Installing oh-my-zsh...".to_string());

    cross_install("zsh").expect("Failed to install zsh");

    let output = Command::new("curl")
        .arg("-o-")
        .arg("https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to curl oh-my-zsh installer");

    let stdout = output.stdout.expect("Failed to get NVM installer stdout");

    Command::new(BASH_SHELL)
        .stdin(Stdio::from(stdout))
        .spawn()
        .expect("Failed install oh-my-zsh script")
        .wait()
        .expect("Failed to wait for oh-my-zsh installer");

    Logger::success("oh-my-zsh installed successfully!".to_string());
}

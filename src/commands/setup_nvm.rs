use std::process::{Command, Stdio};

use crate::common::logger::Logger;

pub fn run() {
    Logger::info("Installing NVM...".to_string());

    let output = Command::new("curl")
        .arg("-o-")
        .arg("https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to curl NVM installer");

    let stdout = output.stdout.expect("Failed to get NVM installer stdout");

    Command::new("bash")
        .stdin(Stdio::from(stdout))
        .spawn()
        .expect("Failed install NVM script");
}

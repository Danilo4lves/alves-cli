use crate::common::constants::{BASHRC_FILE, BASH_SHELL, ZSHRC_FILE, ZSH_SHELL};
use std::process::Command;

pub fn source_env() {
    let home_dir = home::home_dir().expect("Failed to get home directory");
    let zshrc_path = home_dir.join(ZSHRC_FILE);

    let (rc_file, shell) = match zshrc_path.exists() {
        true => (ZSHRC_FILE, ZSH_SHELL),
        false => (BASHRC_FILE, BASH_SHELL),
    };

    let error_msg = format!("Failed to source {}", rc_file);

    Command::new(shell)
        .arg("-c")
        .arg(format!("source {}", rc_file))
        .current_dir(home_dir)
        .spawn()
        .expect(&error_msg)
        .wait()
        .expect(&error_msg);
}

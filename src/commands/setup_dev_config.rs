use crate::common::logger::Logger;

use super::setup_neovim;

pub fn run() {
    Logger::warn(
        "Setup focused on MacOS. If you are using another OS you may encounter some errors."
            .to_string(),
    );

    setup_neovim::run();
}

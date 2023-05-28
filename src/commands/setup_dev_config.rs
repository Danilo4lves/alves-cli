use super::{setup_neovim, setup_zsh, setup_nvm};

pub fn run() {
    setup_zsh::run();
    setup_nvm::run();
    setup_neovim::run();
}

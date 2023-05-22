mod commands;
mod common;

use clap::{ArgEnum, Parser};
use commands::{cpf, setup_neovim};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum CliCommand {
    CpfGenerate,
    CpfValidate,
    SetupNeovim,
}

#[derive(Parser)]
struct Cli {
    #[clap(arg_enum, value_parser)]
    command: CliCommand,
    value: Option<String>,
}

fn main() {
    let Cli { command, value } = Cli::parse();

    match command {
        CliCommand::CpfGenerate => cpf::generate(),
        CliCommand::CpfValidate => cpf::validate(&value),
        CliCommand::SetupNeovim => setup_neovim::run(),
    }
}

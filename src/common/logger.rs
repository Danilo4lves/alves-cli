use colored::Colorize;

pub struct Logger {}

impl Logger {
    pub fn error(msg: String) {
        println!("{}: {}", "ERROR".red().bold(), msg);
    }

    pub fn info(msg: String) {
        println!("{}: {}", "INFO".blue().bold(), msg);
    }

    pub fn warn(msg: String) {
        println!("{}: {}", "WARN".yellow().bold(), msg);
    }
}

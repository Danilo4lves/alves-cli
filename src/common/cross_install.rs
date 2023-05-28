use std::process::{Command, ExitStatus};

pub fn cross_install(app: &str) -> Result<ExitStatus, std::io::Error> {
    match std::env::consts::OS {
        "macos" => Command::new("brew")
            .arg("install")
            .arg(app)
            .spawn()
            .expect(format!("{} to be installed", app).as_str())
            .wait(),
        _ => Command::new("sudo")
            .arg("apt")
            .arg("install")
            .arg("-y")
            .arg(app)
            .spawn()
            .expect(format!("{} to be installed", app).as_str())
            .wait(),
    }
}

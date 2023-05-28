use std::process::Command;

pub fn cross_install(linux_app_name: &str, mac_app_name: &str) {
    match std::env::consts::OS {
        "macos" => {
            Command::new("brew")
                .arg("install")
                .arg(mac_app_name)
                .spawn()
                .expect(format!("{} to be installed", mac_app_name).as_str())
                .wait()
                .expect(format!("{} to be installed", mac_app_name).as_str());
        }
        _ => {
            Command::new("sudo")
                .arg("apt")
                .arg("install")
                .arg("-y")
                .arg(linux_app_name)
                .spawn()
                .expect(format!("{} to be installed", linux_app_name).as_str())
                .wait()
                .expect(format!("{} to be installed", linux_app_name).as_str());
        }
    }
}

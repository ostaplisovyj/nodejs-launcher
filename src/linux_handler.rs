use std::{error::Error, path::PathBuf, process::Command};

use crate::config::LaunchConfig;
pub fn verify_configuration(config: &mut LaunchConfig) {
    let exists = config.is_exists();
    println!("init specified, exists {}", exists);

    if !exists {
        let _path = config.create_config_dir();
        let _file = config.generate_config_file();
        let _editor = open_editor(&config.file_path);
        println!("created launch.json");
    } else {
        println!("config dir exists");
    }
}

pub fn open_editor(file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut editor = Command::new("nano")
        .arg(&file_path)
        .spawn()
        .expect("Failed to laucnh editor");
    let _editor_e_code = editor.wait().expect("failed to exit editor process");

    Ok(())
}

pub fn add_configuration(config: &mut LaunchConfig, name: &str) -> Result<(), Box<dyn Error>> {
    config.add_configuration(name);
    config.save_config_file();
    open_editor(&config.file_path);

    Ok(())
}

use std::{error::Error, path::PathBuf, process::Command};

use crate::config::LaunchConfig;
pub fn verify_configuration(config: &mut LaunchConfig) {
    let exists = config.is_exists();
    println!("init specified, exists {}", exists);

    if !exists {
        let _path = config.create_config_dir();
        let _file = config.generate_config_file();
        let _editor = open_editor(&config.file_path, 1);
        println!("created launch.json. see folder .node_launcher");
    } else {
        println!("config dir exists");
    }
}

pub fn open_editor(file_path: &PathBuf, lines: usize) -> Result<(), Box<dyn Error>> {
    let nano_line_operator = "+";
    let lines_offset = lines - 4; // 4 lines because of default line count of empty configuration
    let arg = format!("{}{}", nano_line_operator, lines_offset);

    let mut editor = Command::new("nano")
        .arg(arg)
        .arg(&file_path)
        .spawn()
        .expect("Failed to launch nano editor, please make sure it's intalled");
    let _editor_e_code = editor.wait().expect("failed to exit editor process");

    Ok(())
}

pub fn add_configuration(config: &mut LaunchConfig, name: &str) -> Result<(), Box<dyn Error>> {
    config.add_configuration(name);
    let lines = config.save_config_file();
    open_editor(&config.file_path, lines.unwrap()).expect("failed opening nano editor");

    Ok(())
}


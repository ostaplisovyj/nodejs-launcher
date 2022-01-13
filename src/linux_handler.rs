use std::{error::Error, path::PathBuf, collections::HashMap, fs::{self, OpenOptions}, process::Command};
use std::io::Write;
use crate::config::Config;
use crate::parser::LauncherConfig;

pub fn verify_configuration() {
    let launcher_path = Config::create(None, None);
    let exists = launcher_path.is_exists();
    println!("init specified, exists {}", exists);

    if !exists {
        let _path = create_config_dir();
        let _editor = open_editor(&launcher_path.get_config_path());
        println!("created launch.json");
    } else {
        println!("config dir exists");
    }

    std::process::exit(0)
}

fn open_editor(file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut editor = Command::new("nano").arg(&file_path).spawn().expect("Failed to laucnh editor");
    let _editor_e_code = editor.wait().expect("failed to exit editor process");

    Ok(())
}

fn create_config_dir() -> Result<PathBuf, Box<dyn Error>> {
    let launcher_path = Config::create(None, None);

    let init_config = LauncherConfig {
        name: String::from("Launch Configuration"),
        script: String::from("./script.js"),
        env: HashMap::new()
    };

    fs::create_dir_all(&launcher_path.json_folder)?;
    let mut file = OpenOptions::new().create_new(true).write(true).append(true).open(&launcher_path.get_config_path()).unwrap();
    let json_string = serde_json::to_string_pretty(&init_config);
    let _file = file.write_all(json_string.unwrap().as_bytes());

    Ok(launcher_path.get_config_path())
}


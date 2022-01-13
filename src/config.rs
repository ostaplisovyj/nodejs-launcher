use std::path::{PathBuf, Path};

pub struct Paths {}

impl Paths {
    const JSON_FOLDER: &'static str = ".node_launcher";
    const CONFIG_FILE_NAME: &'static str = "launch.json";
}

pub struct Config {
    pub json_folder: String,
    pub config_file_name: String
}

impl Config {
    pub fn create(json_folder: Option<&str>, config_file_name: Option<&str>) -> Config {
        let folder = json_folder.unwrap_or(&Paths::JSON_FOLDER);
        let file = config_file_name.unwrap_or(&Paths::CONFIG_FILE_NAME);

        Config {
            json_folder: folder.to_string(),
            config_file_name: file.to_string()
        }
    }

    pub fn get_config_path(&self) -> PathBuf {
        return Path::new("./").join(&self.json_folder).join(&self.config_file_name);        
    }

    pub fn is_exists(&self) -> bool {
        let path: PathBuf = self.get_config_path();

        path.exists()
    }
}


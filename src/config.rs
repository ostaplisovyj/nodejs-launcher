use std::io::{Write, BufReader, BufRead};
use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File, OpenOptions},
    path::PathBuf,
};

use crate::parser::{self, LaunchConfiguration};

pub struct LaunchConfig {
    pub file_name: &'static str,
    pub file_path: PathBuf,
    pub configurations: Vec<Box<LaunchConfiguration>>,
}

impl LaunchConfig {
    pub fn create() -> LaunchConfig {
        LaunchConfig {
            file_name: "launch.json",
            file_path: ["./.node_launcher", "launch.json"].iter().collect(),
            configurations: vec![Box::new(LaunchConfiguration {
                name: "Default".to_string(),
                script: "./script.js".to_string(),
                env: HashMap::new(),
            })],
        }
    }

    pub fn load() -> LaunchConfig {
        let mut launch_config = LaunchConfig::create();
        let configurations: Vec<LaunchConfiguration> =
            parser::ConfigParser::parse(&launch_config.file_path).unwrap();
        launch_config.configurations = configurations
            .iter()
            .map(|configuration| Box::new(configuration.to_owned()))
            .collect();

        launch_config
    }

    pub fn is_exists(&self) -> bool {
        self.file_path.exists()
    }

    pub fn add_configuration(&mut self, name: &str) {
        let default_configuration = LaunchConfiguration {
            name: name.to_string(),
            script: String::from("./script.js"),
            env: HashMap::new(),
        };
        self.configurations.push(Box::new(default_configuration));

        ()
    }

    pub fn find_configuration(&self, name: &str) -> Option<&Box<LaunchConfiguration>> {
        return self.configurations.iter().find(|conf| conf.name == name);
    }

    pub fn generate_config_file(&mut self) -> Result<(), Box<dyn Error>> {
        if self.is_exists() {
            ()
        }

        let path = &self.file_path;
        let _file = File::create(&path)?;
        let _ = &self.save_config_file()?;

        Ok(())
    }

    pub fn save_config_file(&mut self) -> Result<usize, Box<dyn Error>> {
        let mut json_file = OpenOptions::new()
            .write(true)
            .append(false)
            .open(&self.file_path)?;
        let json_string = serde_json::to_string_pretty(&self.configurations);
        {
            let _file = json_file.write_all(json_string.unwrap().as_bytes())?;
        }

        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        let lines = reader.lines().count();

        Ok(lines)
    }

    pub fn create_config_dir(&self) -> Result<Option<()>, Box<dyn Error>> {
        fs::create_dir_all("./.node_launcher")?;

        Ok(None)
    }
}

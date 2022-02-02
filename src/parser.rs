use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::read_to_string, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LaunchConfiguration {
    pub name: String,
    pub script: String,
    pub env: HashMap<String, String>,
}

pub struct ConfigParser {}

impl ConfigParser {
    pub fn parse(file: &PathBuf) -> Result<Vec<LaunchConfiguration>, Box<dyn Error>> {
        let file = read_to_string(&file).expect("failed reading config file.");
        let parsed_json: Vec<LaunchConfiguration> =
            serde_json::from_str(&file).expect("error parsing json");

        Ok(parsed_json)
    }
}

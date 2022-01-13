use std::{collections::HashMap, error::Error, fs::read_to_string};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherConfig {
    pub name: String,
    pub script: String,
    pub env: HashMap<String, String>
}

pub struct JsonParser {}

impl JsonParser {
    pub fn parse(file: &str) -> Result<LauncherConfig, Box<dyn Error>> {
        let file = read_to_string(file).expect("couldn't read config file, please run init command");
        let parsed_json: LauncherConfig = serde_json::from_str(&file).expect("error parsing json");

        Ok(parsed_json)
    }
}


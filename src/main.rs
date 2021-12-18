use std::{fs::read_to_string, env, collections::HashMap, path::Path};
use std::process::Command;

struct JsonParser {}

impl JsonParser {
    fn parse(file: &str) -> HashMap<String, String> {
        let file = read_to_string(file).expect("couldn't read file");
        let json: serde_json::Value = serde_json::from_str(&file).expect("error parsing json");
        let parsed = json.as_object().unwrap().clone();
        let mut map = HashMap::new();

        parsed.iter().for_each(|(key, value)| {
            map.insert(key.clone(), match &value.is_array() {
                true => {
                    value.to_string()
                },
                false => {
                    value.as_str().unwrap().to_string()
                }
            });
        });

        map
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if cfg!(target_os = "linux") {
        println!("Linux!");
        let result = JsonParser::parse(&args[1]);
        println!("{:?}", result);

        Command::new("echo").args(["node", "--version"]).spawn().expect("failed to log node version");
        Command::new("node").envs(&result).arg(Path::new("./test.js")).spawn().expect("failed to run nodejs script");
    } else {
        println!("Hello, world!");
    }
}

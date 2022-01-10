use std::env::var;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::{fs::{self, File}, io::Write};
use std::path::PathBuf;
use std::{fs::read_to_string, env, collections::HashMap, path::Path};
use std::process::{Command, self};
use serde_json::json;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    // short and long flags (-i, --init) will be deduced from the field's name
    #[structopt(short, long)]
    init: bool,

    //// Set speed
    // we don't want to name it "speed", need to look smart
    // #[structopt(short = "v", long = "velocity", default_value = "42")]
    //speed: f64,

    /// Input file
    #[structopt(short, parse(from_os_str))]
    config: Option<PathBuf>,

    //// Output file, stdout if not present
    //#[structopt(parse(from_os_str))]
    //output: Option<PathBuf>,

    //// Where to write the output: to `stdout` or `file`
    //#[structopt(short)]
    //out_type: String,

    //// File name: only required when `out-type` is set to `file`
    //#[structopt(name = "FILE", required_if("out-type", "file"))]
    //file_name: Option<String>,
}



struct JsonParser {}

impl JsonParser {
    fn parse(file: &str) -> HashMap<String, String> {
        let file = read_to_string(file).expect("couldn't read file");
        let json: serde_json::Value = serde_json::from_str(&file).expect("error parsing json");
        let parsed = json.as_object().unwrap().clone();
        let mut map = HashMap::new();

        parsed.into_iter().for_each(|(key, value)| {
            map.insert(key, match &value.is_array() {
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

fn check_config_dir() -> bool {
    let config_dir = Path::new("./.node_launcher/launch.json");
    
    config_dir.exists()
}

fn create_config_dir() -> Result<PathBuf, Box<dyn Error>> {
    let directory = Path::new("./.node_launcher");
    let path = Path::new("./.node_launcher/launch.json");

    // Sample json
    let john = json!({
        "name": "Ostap",
    });
    fs::create_dir_all(&directory)?;
    let mut file = OpenOptions::new().create_new(true).write(true).append(true).open(&path).unwrap();
    let json_string = serde_json::to_string_pretty(&john);
    file.write_all(json_string.unwrap().as_bytes());

    Ok(path.to_path_buf())
}

fn open_editor() -> Result<(), Box<dyn Error>> {
    Command::new("nano").arg(Path::new("./.node_launcher/launch.json")).status().expect("Failed to laucnh editor");

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = Opt::from_args();

    println!("parsed with lib {:?}", opts);

    if cfg!(target_os = "linux") {
        println!("Linux!");

        if opts.init == true {
            let exists = check_config_dir();
            println!("init specified, exists {}", exists);
            if !exists {
                let path = create_config_dir();
                //open_editor();
                println!("created launch.json")
            } else {
                 println!("config dir exists")
            }
            std::process::exit(0)
        }

        let result = JsonParser::parse(&args[1]);
        println!("{:?}", result);

        Command::new("echo").args(["node", "--version"]).spawn().expect("failed to log node version");
        Command::new("node").envs(&result).arg(Path::new(&args[2])).spawn().expect("failed to run nodejs script");
    } else {
        println!("Hello, world!");
    }
}

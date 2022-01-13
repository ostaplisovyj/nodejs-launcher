use structopt::StructOpt;

use crate::launcher::launch_node;
mod parser;
mod launcher;
mod linux_handler;
mod config;

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

    #[structopt(short, long)]
    name: Option<String>,

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

fn main() {
    let opts = Opt::from_args();

    println!("parsed with lib {:?}", opts);

    if cfg!(target_os = "linux") {
        println!("Linux!");

        if opts.init == true {
            linux_handler::verify_configuration();
        }

        let launch_config = parser::JsonParser::parse("./.node_launcher/launch.json").unwrap();
        println!("parsed config: {:?}", launch_config);
        launch_node(&launch_config);

    } else {
        println!("Only Linux is supported by far, sorry.");
    }
}

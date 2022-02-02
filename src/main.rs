mod config;
mod launcher;
mod linux_handler;
mod parser;

use crate::{config::LaunchConfig, launcher::launch_node};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "NodeJS Launcher",
    about = "CLI tool which provides launch configurations for nodejs applications"
)]
struct Opt {
    #[structopt(short, long)]
    init: bool,

    #[structopt(long)]
    new: Option<String>,

    #[structopt(short, long)]
    name: Option<String>,
}

fn main() {
    let opts = Opt::from_args();

    if cfg!(target_os = "windows") {
        println!("Sorry, Windows OS is not yet supported");
        std::process::exit(0)
    }

    if opts.init == true {
        println!("init");
        let mut config: LaunchConfig = LaunchConfig::create();
        linux_handler::verify_configuration(&mut config);
        std::process::exit(0)
    }

    let mut config = LaunchConfig::load();

    if let Some(new_configuration) = opts.new {
        linux_handler::verify_configuration(&mut config);
        linux_handler::add_configuration(&mut config, &new_configuration);
        std::process::exit(0)
    }

    if let Some(name) = opts.name {
        let config = config.find_configuration(&name);
        println!("executing {}", name);
        launch_node(&config.unwrap());
    } else {
        println!("configuration name not specified, launching first configuration");
        launch_node(&config.configurations[0]);
    }
}

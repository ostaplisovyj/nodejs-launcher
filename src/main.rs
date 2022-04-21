mod config;
mod launcher;
mod linux_handler;
mod parser;
use crate::{config::LaunchConfig, launcher::launch_node};
use structopt::StructOpt;
use dialoguer::{Select};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "NodeJS Launcher",
    about = "CLI tool which provides launch configurations for nodejs applications"
)]
enum Opt {
    /// inits the .launch.json file which will be used to store configurations
    Init {},

    /// opens a prompt to specify the launch configuration 
    Run {},

    /// opens the command line editor (nano) to edit launch configurations
    Edit {},

    /// adds new launch configuration
    Add {}
}

fn main() -> std::io::Result<()> {
    let opts = Opt::from_args();

    if cfg!(target_os = "windows") {
        println!("Sorry, Windows OS is not yet supported");
        std::process::exit(0)
    }

    match opts {
        Opt::Init {} => {
            handle_init();
        }
        Opt::Run {} => {
            handle_run_configuration();
        }
        Opt::Edit {} => {
            handle_edit_configuration();
        }
        Opt::Add {} => {
            handle_add_configuration();
        }
    }

    std::process::exit(0)
}

fn handle_init() {
    println!("init");
    let mut config: LaunchConfig = LaunchConfig::create();
    linux_handler::verify_configuration(&mut config);
}

fn handle_run_configuration() {
    let config = LaunchConfig::load();
    let items: Vec<&str> = config
        .configurations
        .iter()
        .map(|conf| conf.name.as_ref())
        .collect();

    let selection: Result<usize, std::io::Error> = Select::new()
        .with_prompt("Please select nodejs configuration")
        .items(&items)
        .default(0)
        .interact();

    let configuration = config.find_configuration(&items[selection.unwrap()]);
    println!("executing {:?}", configuration);
    launch_node(&configuration.unwrap());
}

fn handle_edit_configuration() {
    let config = LaunchConfig::load();
    linux_handler::open_editor(&config.file_path, 4).expect("failed to open nano");
}

fn handle_add_configuration() {
    let mut config = LaunchConfig::load();
    linux_handler::add_configuration(&mut config, "New Configuration").expect("failed to add configuration");
}

use crate::parser::LaunchConfiguration;
use std::path::Path;
use std::process::Command;

pub fn launch_node(launch_config: &LaunchConfiguration) {
    let mut child_version = Command::new("node")
        .args(["--version"])
        .spawn()
        .expect("failed to log node version");
    let _vecode = child_version.wait().expect("Failed to exit version");

    let mut child = Command::new("node")
        .envs(&launch_config.env)
        .arg(Path::new(&launch_config.script))
        .spawn()
        .expect("failed to run nodejs script");
    let _ecode = child.wait().expect("failed to wait on child");
}

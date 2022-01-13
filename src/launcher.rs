use std::process::Command;
use std::path::Path;
use crate::parser::LauncherConfig;

pub fn launch_node(launch_config: &LauncherConfig) {
    let mut child_version = Command::new("node").args(["--version"]).spawn().expect("failed to log node version");
    let _vecode = child_version.wait().expect("Failed to exit version");

    let mut child = Command::new("node").envs(&launch_config.env).arg(Path::new(&launch_config.script)).spawn().expect("failed to run nodejs script");
    let _ecode = child.wait().expect("failed to wait on child");
}

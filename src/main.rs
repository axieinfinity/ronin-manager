mod cmd;
mod util;

use std::fs::{self};
use std::path::PathBuf;
use std::error::Error;

fn main() {
    set_working_dir().unwrap();
    cmd::commander().run();
}

fn set_working_dir() -> Result<(), Box<dyn Error>> {
    let mut path = std::env::current_exe().unwrap();
    // Get the directory
    path.pop();

    let dir_name = path.file_name().unwrap();

    // In case of cargo run
    if check_config(&path).is_err() && (dir_name.eq("debug") || dir_name.eq("release")) {
        path.pop();
        path.pop();
    }

    check_config(&path)
}

fn check_config(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut has_env_file = false;
    let mut has_config_dir = false;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.path().is_file() && entry.file_name().eq(".env") {
            has_env_file = true;
        }
        if entry.path().is_dir() && entry.file_name().eq("config") {
            has_config_dir = true;
        }
    }
    if !has_env_file {
        Err(".env file is not available".into())
    } else if !has_config_dir{
        Err("config directory is not available".into())
    } else {
        Ok(())
    }
}

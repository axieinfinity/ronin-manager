use std::path::Path;

use clap_nested::Command;
use duct;

pub fn cmd<'a>() -> Command<'a, ()> {
    Command::new("upgrade")
        .description("Upgrade node manager to latest version")
        .runner(|_env, _matches| {
            let (should_update, latest_version) = verify_latest_version();
            if should_update {
                let mut current_path = std::env::current_exe().unwrap();
                // Get the directory
                current_path.pop();
                let old_dir = current_path.as_path();

                // Go to home directory
                std::env::set_current_dir(Path::new(&std::env::var("HOME").unwrap())).unwrap();
                download_latest_version();
                let new_path = format!(
                    "{}/pkg-ronin-manager-{}",
                    &std::env::var("HOME").unwrap(),
                    latest_version
                );
                let new_dir = Path::new(&new_path);

                copy_current_config(old_dir, new_dir);

                std::env::set_current_dir(old_dir).unwrap();
                stop_services();

                std::env::set_current_dir(new_dir).unwrap();
                start_new_services();
                delete_old_location(old_dir);
            }
            Ok(())
        })
}

pub fn verify_latest_version() -> (bool, String) {
    let current_version = clap::crate_version!();
    println!("Current version = {}", current_version);
    let latest_version = duct::cmd(
        "curl",
        vec!["-s", "https://chain.skymavis.one/ronin-latest-version"],
    )
    .stdout_capture()
    .read()
    .unwrap();

    println!("Latest version = {}", latest_version);
    if current_version == latest_version {
        println!("Already at the latest version. No update required!");
        (false, latest_version)
    } else {
        println!("Updating to version {}", latest_version);
        (true, latest_version)
    }
}

pub fn download_latest_version() {
    println!("Download and save latest version");

    duct::cmd(
        "curl",
        vec![
            "-s",
            "-L",
            "-O",
            "https://chain.skymavis.one/downloads/ronin-manager-linux-latest.tar.gz",
        ],
    )
    .then(duct::cmd(
        "tar",
        vec!["fx", "ronin-manager-linux-latest.tar.gz"],
    ))
    .then(duct::cmd("rm", vec!["ronin-manager-linux-latest.tar.gz"]))
    .run()
    .unwrap();
}

pub fn copy_current_config(old_dir: &Path, new_dir: &Path) {
    println!("Copying current config to new folder");
    duct::cmd(
        "cp",
        vec![
            format!("{}/.env", old_dir.to_str().unwrap()),
            format!("{}/.env", new_dir.to_str().unwrap()),
        ],
    )
    .run()
    .unwrap();
}

pub fn stop_services() {
    duct::cmd("docker-compose", vec!["down"]).run().unwrap();
}

pub fn start_new_services() {
    duct::cmd(
        "docker-compose",
        vec!["--env-file", "~/.skymavis/.env", "up", "-d"],
    )
    .run()
    .unwrap();
}

pub fn delete_old_location(old_dir: &Path) {
    duct::cmd("rm", vec!["-rf", old_dir.to_str().unwrap()])
        .run()
        .unwrap();
}

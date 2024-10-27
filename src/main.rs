mod config;
mod service;
use daemonize::Daemonize;
use std::fs::File;

fn main() {
    let stdout = File::create("/tmp/eirship.out").unwrap();
    let stderr = File::create("/tmp/eirship.err").unwrap();
    let daemonize = Daemonize::new()
        .pid_file("/tmp/eirship.pid")
        .chown_pid_file(true)
        .working_directory("/")
        .user("nobody")
        .group("nogroup")
        .stdout(stdout)
        .stderr(stderr);
    match daemonize.start() {
        Ok(_) => println!("Daemon started successfully"),
        Err(e) => eprintln!("Error starting daemon: {}", e),
    }
}

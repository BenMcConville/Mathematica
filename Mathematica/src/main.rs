pub mod app;
use std::process::Command;

fn main() {
    Command::new("ls")
        .arg("-a")
        .output()
        .expect("Failed to execute command");
    app::run();
}

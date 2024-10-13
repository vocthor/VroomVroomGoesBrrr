mod controllers;
mod events;
mod orchestrator;
mod server;

use crate::controllers::cli::start_cli_controller;
use crate::controllers::web::start_web_controller;

#[tokio::main]
/// The main entry point of the application.
async fn main() {
    start_web_controller(3000);
    start_cli_controller();

    println!("Application started");

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
/// Stops the server and exits the process.
pub fn stop() {
    println!("Server stopped");
    std::process::exit(0);
}

mod controllers;
mod events;
mod orchestrator;
mod server;

use crate::controllers::cli::start_cli_controller;
use crate::controllers::web::start_web_controller;
use crate::events::models::Event;
use crate::orchestrator::orchestrator::Orchestrator;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[tokio::main]
/// The main entry point of the application.
async fn main() {
    // create a queue using Arc
    let event_queue: Arc<Mutex<VecDeque<Event>>> = Arc::new(Mutex::new(VecDeque::new()));
    start_web_controller(3000);
    start_cli_controller(event_queue.clone());
    let orchestrator = Orchestrator::new(event_queue.clone());
    orchestrator.start();

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

use std::thread::Thread;
use crate::controllers::web::start_web_server;

mod controllers;

#[tokio::main]
async fn main() {
    start_web_server(3000);
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

pub fn stop(){
    println!("Server stopped");
    std::process::exit(0);
}

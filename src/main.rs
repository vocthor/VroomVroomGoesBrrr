mod controllers;

use crate::controllers::web::start_web_server;

#[tokio::main]
/// The main entry point of the application.
async fn main() {
    start_web_server(3000);

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
/// Stops the server and exits the process.
pub fn stop(){
    println!("Server stopped");
    std::process::exit(0);
}

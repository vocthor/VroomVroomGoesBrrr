use std::collections::VecDeque;
use std::io::Read;
use std::path::PathBuf;
use std::os::unix::net::{UnixStream, UnixListener};
use std::sync::{Arc, Mutex};
use common_cli_messages::{deserialize_cli_message, get_cli_socket_path, CliMessage};
use tokio::sync::oneshot::{Sender, Receiver};
use crate::events::models::{Event, StartEvent, StartEventResponse};

pub fn start_cli_controller(queue: Arc<Mutex<VecDeque<Event>>>) {
    tokio::spawn(async move {
        start(queue).await.expect("Failed to start CLI controller");
    });
}

async fn start(queue: Arc<Mutex<VecDeque<Event>>>) -> std::io::Result<()> {
    std::fs::remove_file(get_cli_socket_path()).ok();

    // bind to the socket
    let listener = UnixListener::bind(get_cli_socket_path()).expect("Failed to bind to socket");

    // accept connections and process them
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let queue_clone = Arc::clone(&queue);
                tokio::task::spawn_blocking(move || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async move {
                        handle_client(stream, queue_clone).await;
                    });
                });
            }
            Err(err) => {
                eprintln!("Error accepting connection: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

async fn handle_client(mut stream: UnixStream, queue: Arc<Mutex<VecDeque<Event>>>) {
    let mut buf = [0; 1024];
    println!("Handling client");
    // Read from the stream
    match stream.read(&mut buf) {
        Ok(n) if n > 0 => {
            // Clean the buffer using filter
            let buffered_string = trim_buffer(&buf[..n]);
            println!("Received data: {:?}", buffered_string);
            // Deserialize the JSON
            match deserialize_cli_message(&buffered_string) {
                Ok(message) => {
                    println!("Received message: {:?}", message);
                    match message {
                        CliMessage::StartServer(msg) => {
                            println!("Starting server: {:?}", msg);
                            let (resolver, receiver): (Sender<StartEventResponse>, Receiver<StartEventResponse>) = tokio::sync::oneshot::channel();
                            {
                                let mut event_queue = queue.lock().unwrap();
                                event_queue.push_back(
                                    Event::StartEvent(
                                        StartEvent {
                                            name: msg.name,
                                            cfg_server_path: PathBuf::from(msg.cfg_server_path).into_boxed_path(),
                                            cfg_tracklist_path: PathBuf::from(msg.cfg_tracklist_path).into_boxed_path(),
                                            resolver: resolver,
                                        }
                                    )
                                );
                            } // MutexGuard is dropped here

                            let response = receiver.await.expect("Failed to receive response");
                        }
                        CliMessage::StopServer(msg) => {
                            println!("Stopping server: {:?}", msg);
                        }
                        CliMessage::GetServerInfo(msg) => {
                            println!("Getting server info: {:?}", msg);
                        }
                    }
                }
                Err(err) => {
                    println!("Error deserializing JSON: {:?}", err);
                }
            }
        }
        Ok(_) => {
            println!("No data received");
        }
        Err(err) => {
            println!("Error reading from stream: {:?}", err);
        }
    }
}

fn trim_buffer(buf: &[u8]) -> String {
    // Utilise filter pour garder uniquement les caractères valides
    let filtered: String = buf.iter()
        .filter(|&&b| b != 0 && b != b'\n') // Retire les caractères nuls et les retours à la ligne
        .map(|&b| b as char)               // Convertit les octets en caractères
        .collect();                        // Collecte en une String

    filtered
}
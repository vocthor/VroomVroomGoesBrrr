use std::io::Read;
use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};
use serde::Deserialize;
use crate::controllers::models::CliCommand;

pub fn start_cli_controller(){
    tokio::spawn(async move {
        start().expect("Failed to start CLI controller");
    });
}
fn start()-> std::io::Result<()> {
    let path = "/tmp/vroom-vroom.sock";
    // remove the socket file if it already exists
    std::fs::remove_file(path).ok();

    // bind to the socket
    let listener = UnixListener::bind(path).expect("Failed to bind to socket");

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                break;
            }
        }
    }
    Ok(())
}

fn handle_client(mut stream: UnixStream) {
    let mut buf = [0; 1024];

    // Lire le flux
    match stream.read(&mut buf) {
        Ok(n) if n > 0 => {
            // Nettoyer le buffer en utilisant filter
            let buffered_string = trim_buffer(&buf[..n]);

            // Désérialiser le JSON
            match serde_json::from_str::<CliCommand>(&buffered_string) {
                Ok(command) => {
                    println!("Received: \nMessage Type: {}\nMessage: {}", command.get_message_type(), command.get_message());
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


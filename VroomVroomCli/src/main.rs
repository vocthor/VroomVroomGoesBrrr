use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use clap::{Arg, Command};
use common_cli_messages::{deserialize_cli_response, get_cli_response_socket_path, get_cli_message_socket_path, trim_buffer, CliMessage, StartServerCliMessage, StopServerCliMessage, GetServerInfoCliMessage};



#[tokio::main]
async fn main() {
    // Définition des commandes
    let matches = Command::new("trackmania-cli")
        .version("1.0")
        .about("CLI pour déployer des serveurs Trackmania")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("server")
                .about("Commandes pour les serveurs")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("start")
                        .about("Démarre un serveur")
                        .arg(Arg::new("name")
                            .help("Nom du serveur")
                            .required(true)
                        )
                )
                .subcommand(
                    Command::new("status")
                        .about("Affiche le status d'un serveur")
                        .arg(Arg::new("id")
                            .help("Nom du serveur")
                            .required(true)
                        )
                )
                .subcommand(
                    Command::new("delete")
                        .about("Supprime un serveur")
                        .arg(Arg::new("id")
                            .help("Nom du serveur")
                            .required(true)
                        )
                )
                .subcommand(
                    Command::new("list")
                        .about("Liste tous les serveurs")
                )
        ).get_matches();

    // Gestion des actions pour 'server'
    if let Some(server_matches) = matches.subcommand_matches("server") {
        match server_matches.subcommand() {
            Some(("start", sub_matches)) => {
                let server_name: String = sub_matches.get_one::<String>("name").expect("Le nom du serveur est obligatoire").to_string();
                let cli_message = CliMessage::StartServerCliMessage(StartServerCliMessage {
                    name: server_name.clone(),
                    map_path: "map_path".to_string(),
                    cfg_server_path: "cfg_server_path".to_string(),
                    cfg_tracklist_path: "cfg_tracklist_path".to_string(),
                });

                send_message(cli_message);
                wait_for_response();
            }
            Some(("delete", sub_matches)) => {
                let server_id: String = sub_matches.get_one::<String>("id").expect("Le nom du serveur est obligatoire").to_string();
                let cli_message = CliMessage::StopServerCliMessage(StopServerCliMessage {
                    id: server_id.parse::<u32>().unwrap(),
                });
                send_message(cli_message);
                wait_for_response();
            }
            Some(("status", sub_matches)) => {
                let server_id: String = sub_matches.get_one::<String>("id").expect("Le nom du serveur est obligatoire").to_string();
                let cli_message = CliMessage::GetServerInfoCliMessage(GetServerInfoCliMessage {
                    id: server_id.parse::<u32>().unwrap(),
                });
                send_message(cli_message);
                wait_for_response();
            }
            Some(("list", _)) => {
                let cli_message = CliMessage::ListServerCliMessage {};
                send_message(cli_message);
                wait_for_response();
            },
            _ => eprintln!("Commande serveur non reconnue"),
        }
    }
}

pub fn wait_for_response() {
    std::fs::remove_file(get_cli_response_socket_path()).ok();

    // bind to the socket
    let listener = UnixListener::bind(get_cli_response_socket_path()).expect("Failed to bind to socket");

    // accept connections and process them
    let mut result = listener.accept();

    match result {
        Ok((mut stream, _)) => {
            let mut buf: [u8; 1024] = [0; 1024];
            match stream.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let response = deserialize_cli_response(&trim_buffer(&buf[..n])).unwrap();
                    println!("Received data: {:?}", response);
                }
                Ok(_) => {
                    println!("Received empty data");
                }
                Err(err) => {
                    eprintln!("Error reading from stream: {:?}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error accepting connection: {:?}", err);
        }
    }
}

pub fn send_message(message: CliMessage) {
    let mut stream = UnixStream::connect(get_cli_message_socket_path());
    match stream {
        Ok(mut stream) => {
            let serialized = common_cli_messages::serialize_cli_message(&message).unwrap();
            stream.write(serialized.as_bytes()).unwrap();
        }
        Err(err) => {
            eprintln!("Impossible de se connecter au socket: {:?}", err);
        }
    }
}
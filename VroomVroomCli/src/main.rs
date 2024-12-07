use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::exit;
use clap::{Arg, ArgMatches, Command};
use common_cli_messages::{deserialize_cli_response, get_cli_message_socket_path, trim_buffer, CliMessage, StartServerCliMessage, StopServerCliMessage, GetServerInfoCliMessage, ListServerCliMessage};


#[tokio::main]
async fn main() {
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
                        .arg(
                            Arg::new("cfg_server_path")
                                .short('s')
                                .long("cfg_server_path")
                                .help("Chemin vers la configuration du serveur")
                                .required(true)
                        )
                        .arg(
                            Arg::new("cfg_tracklist_path")
                                .short('t')
                                .long("cfg_tracklist_path")
                                .help("Chemin vers la configuration de la tracklist")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("status")
                        .about("Affiche le status d'un serveur")
                        .arg(Arg::new("id")
                            .help("Identifiant du serveur")
                            .required(true)
                        )
                )
                .subcommand(
                    Command::new("stop")
                        .about("Arrete et supprime un serveur")
                        .arg(Arg::new("id")
                            .help("Identifiant du serveur")
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
                handle_start_command(sub_matches);
            }
            Some(("stop", sub_matches)) => {
                handle_stop_command(sub_matches);
            }
            Some(("status", sub_matches)) => {
                handle_status_command(sub_matches);
            }
            Some(("list", _)) => {
                handle_list_command();
            }
            _ => eprintln!("Commande serveur non reconnue"),
        }
    }
}

/// Handle the list command
/// This function
/// - creates a random socket path that will be used to receive the response from the server
/// - sends a ListServerCliMessage to the server
/// - waits for a response from the server
///
/// # Arguments
/// * `sub_matches` - The sub_matches that contains the arguments for the list command
fn handle_list_command() {
    let socket_path = create_random_socket_path();
    let cli_message = CliMessage::ListServerCliMessage(
        ListServerCliMessage {
            response_socket_path: socket_path.clone(),
        }
    );
    send_message(cli_message);
    wait_for_response(socket_path);
}

/// Handle the status command
/// This function
/// - extracts the server name from the sub_matches
/// - creates a random socket path that will be used to receive the response from the server
/// - sends a GetServerInfoCliMessage to the server
/// - waits for a response from the server
///
/// # Arguments
/// * `sub_matches` - The sub_matches that contains the arguments for the status command
fn handle_status_command(sub_matches: &ArgMatches) {
    let server_id: String = sub_matches.get_one::<String>("id").expect("Le nom du serveur est obligatoire").to_string();
    let socket_path = create_random_socket_path();

    let cli_message = CliMessage::GetServerInfoCliMessage(GetServerInfoCliMessage {
        id: server_id.parse::<u32>().unwrap(),
        response_socket_path: socket_path.clone()
    });
    send_message(cli_message);
    wait_for_response(socket_path);
}

/// Handle the stop command
/// This function
/// - extracts the server name from the sub_matches
/// - creates a random socket path that will be used to receive the response from the server
/// - sends a StopServerCliMessage to the server
/// - waits for a response from the server
///
/// # Arguments
/// * `sub_matches` - The sub_matches that contains the arguments for the stop command
fn handle_stop_command(sub_matches: &ArgMatches) {
    let server_id: String = sub_matches.get_one::<String>("id").expect("Le nom du serveur est obligatoire").to_string();
    let socket_path = create_random_socket_path();

    let cli_message = CliMessage::StopServerCliMessage(StopServerCliMessage {
        id: server_id.parse::<u32>().unwrap(),
        response_socket_path: socket_path.clone(),
    });
    send_message(cli_message);
    wait_for_response(socket_path);
}

/// Handle the start command
/// This function
/// - extracts the server name, the server configuration path and the tracklist configuration path from the sub_matches
/// - creates a random socket path that will be used to receive the response from the server
/// - sends a StartServerCliMessage to the server
/// - waits for a response from the server
///
/// # Arguments
/// * `sub_matches` - The sub_matches that contains the arguments for the start command
fn handle_start_command(sub_matches: &ArgMatches) {
    let server_name: String = sub_matches.get_one::<String>("name").expect("Le nom du serveur est obligatoire").to_string();
    let cfg_server_path: String = sub_matches.get_one::<String>("cfg_server_path").expect("Le chemin vers la configuration du server est obligatoire").to_string();
    let cfg_tracklist_path: String = sub_matches.get_one::<String>("cfg_tracklist_path").expect("Le chemin vers la configuration de la tracklist est obligatoire").to_string();
    let socket_path = create_random_socket_path();
    let cli_message = CliMessage::StartServerCliMessage(StartServerCliMessage {
        name: server_name.clone(),
        cfg_server_path: cfg_server_path,
        cfg_tracklist_path: cfg_tracklist_path,
        response_socket_path: socket_path.clone(),
    });

    send_message(cli_message);
    wait_for_response(socket_path);
}

/// Wait for a response from the server
/// This function will block until a response is received
///
/// # Arguments
/// * `socket_path` - The path to the socket that will be used to receive the response
///
/// # Errors
///
/// This function will exit the process if it fails to bind to the socket or if it fails to accept a connection
///
fn wait_for_response(socket_path : String) {
    std::fs::remove_file(&socket_path).ok();

    // bind to the socket
    let listener = UnixListener::bind(&socket_path).expect("Failed to bind to socket");

    // accept connections and process them
    let result = listener.accept();

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
                    exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("Error accepting connection: {:?}", err);
            exit(1);
        }
    }
}

/// Create a random socket path
/// This socket will be used to receive responses from the server
///
/// # Returns
///  A string representing the path to the socket
fn create_random_socket_path() -> String {
    let socket_path = format!("/tmp/vroom-vroom-response-{}.sock", rand::random::<u32>());
    std::fs::remove_file(&socket_path).ok();
    socket_path
}

/// Send a message to the server using the socket
///
/// # Arguments
/// * `message` - The message to send to the server
///
/// # Errors
///
/// This function will exit the process if it fails to connect to the socket
fn send_message(message: CliMessage) {
    let stream = UnixStream::connect(get_cli_message_socket_path());
    match stream {
        Ok(mut stream) => {
            let serialized = common_cli_messages::serialize_cli_message(&message).unwrap();
            stream.write(serialized.as_bytes()).unwrap();
        }
        Err(err) => {
            eprintln!("Impossible de se connecter au socket: {:?}", err);
            exit(1);
        }
    }
}
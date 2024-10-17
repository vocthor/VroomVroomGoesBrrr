use std::io::Write;
use std::os::unix::net::UnixStream;
use clap::{Arg, ArgMatches, Command};
use common_cli_messages::{get_cli_socket_path, CliMessage, StartServerCliMessage};

fn main() {
    // Définition des commandes
    let matches = Command::new("trackmania-cli")
        .version("1.0")
        .about("CLI pour déployer des serveurs Trackmania")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // Définition des sous-commandes pour 'server'
        .subcommand(
            Command::new("server")
                .about("Gérer les serveurs")
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
                    Command::new("list")
                        .about("Liste tous les serveurs")
                )
                .subcommand(
                    Command::new("delete")
                        .about("Supprime un serveur")
                        .arg(Arg::new("name")
                            .help("Nom du serveur")
                            .required(true)
                        )
                )
        )
        // Définition des sous-commandes pour 'orchestrator'
        .subcommand(
            Command::new("orchestrator")
                .about("Gérer l'orchestrateur")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("start")
                        .about("Démarre l'orchestrateur")
                )
                .subcommand(
                    Command::new("stop")
                        .about("Arrête l'orchestrateur")
                )
                .subcommand(
                    Command::new("status")
                        .about("Affiche le statut de l'orchestrateur")
                )
        ).get_matches();

    // Gestion des actions pour 'server'
    if let Some(server_matches) = matches.subcommand_matches("server") {
        match server_matches.subcommand() {
            Some(("start", sub_matches)) => {
                let server_name: String = sub_matches.get_one::<String>("name").expect("Le nom du serveur est obligatoire").to_string();
                let cli_message = CliMessage::StartServer(StartServerCliMessage {
                    name: server_name.clone(),
                    map_path: "map_path".to_string(),
                    cfg_server_path: "cfg_server_path".to_string(),
                    cfg_tracklist_path: "cfg_tracklist_path".to_string(),
                });

                let mut stream = UnixStream::connect(get_cli_socket_path());
                match stream {
                    Ok(mut stream) => {
                        let serialized = common_cli_messages::serialize_cli_message(&cli_message).unwrap();
                        stream.write(serialized.as_bytes()).unwrap();
                    }
                    Err(err) => {
                        eprintln!("Impossible de se connecter au socket: {:?}", err);
                    }
                }

                println!("Démarrage du serveur {:?}", server_name);
            }
            Some(("list", _)) => println!("Liste des serveurs"),
            Some(("delete", sub_matches)) => {
                let server_name: String = sub_matches.get_one::<String>("name").expect("Le nom du serveur est obligatoire").to_string();
                println!("Suppression du serveur {:?}", server_name);
            }
            _ => eprintln!("Commande serveur non reconnue"),
        }
    }

    // Gestion des actions pour 'orchestrator'
    if let Some(orchestrator_matches) = matches.subcommand_matches("orchestrator") {
        match orchestrator_matches.subcommand() {
            Some(("start", _)) => println!("Démarrage de l'orchestrateur"),
            Some(("stop", _)) => println!("Arrêt de l'orchestrateur"),
            Some(("status", _)) => println!("Statut de l'orchestrateur"),
            _ => eprintln!("Commande orchestrator non reconnue"),
        }
    }
}
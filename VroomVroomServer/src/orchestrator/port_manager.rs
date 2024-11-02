use crate::server::server::Server;
use std::collections::HashSet;

// Le gestionnaire de ports
pub struct PortManager {
    available_ports: HashSet<u16>, // Ports libres
}

impl PortManager {
    // Initialise le gestionnaire avec une plage de ports disponibles
    pub fn new(start_port: u16, end_port: u16) -> Self {
        let available_ports: HashSet<u16> = (start_port..=end_port).collect();
        PortManager { available_ports }
    }

    // Attribue un port à un serveur
    pub fn get_port_for_server(&mut self) -> Option<u16> {
        // Vérifie s'il existe un port libre
        if let Some(&port) = self.available_ports.iter().next() {
            // Retire le port de la liste des ports disponibles
            self.available_ports.remove(&port);
            // Assigne le port au serveur
            Some(port)
        } else {
            None // Pas de ports disponibles
        }
    }

    // Libère un port assigné à un serveur
    pub fn release_port(&mut self, server: &Server) {
        // Ajoute le port libéré à la liste des ports disponibles
        self.available_ports.insert(server.get_port());
    }

    // // Affiche l'état actuel des ports assignés
    // pub fn print_assigned_ports(&self) {
    //     for (server, port) in &self.assigned_ports {
    //         println!(
    //             "Serveur {:?} ({:?}) utilise le port {}",
    //             server.get_name(),
    //             server.get_id(),
    //             port
    //         );
    //     }
    // }
}

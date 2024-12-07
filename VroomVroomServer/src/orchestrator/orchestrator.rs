use crate::events::models::{Event, StartEvent, StartEventResponse, StopEvent, StopEventResponse};
use crate::orchestrator::port_manager::PortManager;
use crate::server::server::Server;
use core::str;
use fs_extra::dir::{copy, CopyOptions};
use quick_xml::events::BytesText;
use quick_xml::reader::Reader;
use quick_xml::Writer;
use rand::Rng;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{
    collections::{HashMap, VecDeque},
    thread,
};

const SERVER_ROOT_PATH: &str = "./compose/";
const TEMPLATE_SERVER_PATH: &str = "./compose/template/";

pub struct Orchestrator {
    servers: HashMap<u32, Server>,
    event_queue: Arc<Mutex<VecDeque<Event>>>,
    port_manager: PortManager,
}

impl Orchestrator {
    pub fn new(event_queue: Arc<Mutex<VecDeque<Event>>>, port_manager: PortManager) -> Self {
        return Orchestrator {
            servers: HashMap::new(),
            event_queue,
            port_manager,
        };
    }

    // Starts the orchestrator in a new thread.
    pub fn start(mut self) {
        thread::spawn(move || loop {
            let event = {
                let mut queue = self.event_queue.lock().unwrap();
                if !queue.is_empty() {
                    Some(queue.pop_front().unwrap())
                } else {
                    None
                }
            };

            if let Some(event) = event {
                self.process_queue(event);
            }
        });
    }

    fn process_queue(&mut self, event: Event) {
        match event {
            Event::StartEvent(StartEvent {
                name,
                cfg_server_path,
                cfg_tracklist_path,
                resolver,
            }) => {
                println!("Orchestrator processing event : StartEvent");
                let id = self.create_server(name, cfg_server_path, cfg_tracklist_path);
                let successful_server_start = self.start_server(id);
                if successful_server_start {
                    let _ = resolver.send(StartEventResponse { id });
                }
            }
            Event::StopEvent(StopEvent { id, resolver }) => {
                println!("Orchestrator processing event : StopEvent");
                self.stop_server(id);
                let _ = resolver.send(StopEventResponse {});
            }
            _ => {}
        }
    }

    fn create_server(
        &mut self,
        name: String,
        cfg_server_path: Box<Path>,
        cfg_tracklist_path: Box<Path>,
    ) -> u32 {
        // Server Constants
        let id: u32 = rand::thread_rng().gen::<u32>(); // Incha'Allah ya pas de conflit
        let port = self.port_manager.get_port_for_server().unwrap();

        // TODO Opération de recopitage
        // Server files configuration
        create_server_dir(id);
        copy_server_configs(id, &cfg_server_path, &cfg_tracklist_path);
        let mut fields = HashMap::new();
        let binding = port.to_string();
        fields.insert("server_port", binding.as_str());
        modify_xml(&cfg_server_path, fields);

        let serv = Server::new(id, name, port);
        self.servers.insert(id, serv);
        println!("Orchestrator server created : {}", id);
        return id;
    }

    fn start_server(&self, id: u32) -> bool {
        let serv = self.servers.get(&id).unwrap();
        return serv.start_server();
    }

    fn stop_server(&mut self, id: u32) {
        let serv = self.servers.get(&id).unwrap();
        self.port_manager.release_port(serv);
        serv.stop_server();
    }
}

fn create_server_dir(id: u32) {
    let destination_path = Path::new(SERVER_ROOT_PATH).join(id.to_string());

    // Vérifie si le dossier destination existe, sinon le crée
    if !destination_path.exists() {
        fs::create_dir_all(destination_path.clone());
    }

    let mut options = CopyOptions::new();
    options.copy_inside = true; // Copier le contenu au lieu du dossier racine
    copy(TEMPLATE_SERVER_PATH, destination_path, &options);
}

fn copy_server_configs(id: u32, cfg_server_path: &Box<Path>, cfg_tracklist_path: &Box<Path>) {
    let destination_path = Path::new(SERVER_ROOT_PATH).join(id.to_string());

    // Configurer les options pour la copie
    let mut options = CopyOptions::new();
    options.overwrite = true; // Remplace les fichiers existants

    copy(cfg_server_path, destination_path.clone(), &options);
    copy(
        cfg_tracklist_path,
        destination_path.join("maps/MatchSettings"),
        &options,
    );
}

fn modify_xml(xml_file: &Box<Path>, fields: HashMap<&str, &str>) {
    let mut reader = Reader::from_file(xml_file).unwrap();
    reader.config_mut().trim_text(true);

    // Préparer un fichier temporaire pour écrire le nouveau contenu
    let temp_file = File::create("/tmp/temp.xml").unwrap();
    let writer = BufWriter::new(temp_file);
    let mut xml_writer = Writer::new(writer);

    // Pour eviter les conflits avec nos Event
    use quick_xml::events::Event;

    // Lire et écrire en modifiant le champ ciblé
    let mut buf = Vec::new();
    while let Ok(event) = reader.read_event_into(&mut buf) {
        match event {
            // Trouver le champ et modifier sa valeur
            Event::Start(ref e)
                if fields.contains_key(str::from_utf8(e.name().as_ref()).unwrap()) =>
            {
                xml_writer.write_event(Event::Start(e.clone())).unwrap();
                xml_writer.write_event(Event::Text(BytesText::new(
                    fields
                        .get(str::from_utf8(e.name().as_ref()).unwrap())
                        .unwrap(),
                )));
            }
            Event::End(ref e)
                if fields.contains_key(str::from_utf8(e.name().as_ref()).unwrap()) =>
            {
                xml_writer.write_event(Event::End(e.clone()));
            }
            _ => {
                // Copier tous les autres événements sans modification
                xml_writer.write_event(event.clone());
            }
        }
        buf.clear();
    }
}

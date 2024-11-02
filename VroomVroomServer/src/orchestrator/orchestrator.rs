use crate::events::models::{Event, StartEvent, StartEventResponse, StopEvent, StopEventResponse};
use crate::orchestrator::port_manager::PortManager;
use crate::server::server::Server;
use rand::Rng;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{
    collections::{HashMap, VecDeque},
    thread,
};

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
        // TODO Opération de recopitage

        let port = self.port_manager.get_port_for_server().unwrap();

        let id: u32 = rand::thread_rng().gen::<u32>();
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

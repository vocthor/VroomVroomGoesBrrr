use crate::events::models::{Event, StartEvent, StartEventResponse, StopEvent};
use crate::server::server::Server;
use rand::Rng;
use std::path::Path;
// 0.8.5
use std::{
    collections::{HashMap, VecDeque},
    thread,
};

pub struct Orchestrator {
    servers: HashMap<u32, Server>,
    event_queue: VecDeque<Event>,
}

impl Orchestrator {
    pub fn new() -> Self {
        return Orchestrator {
            servers: HashMap::new(),
            event_queue: VecDeque::new(),
        };
    }
    /// Starts the orchestrator in a new thread.
    pub fn start(mut self) {
        thread::spawn(move || loop {
            if !self.event_queue.is_empty() {
                self.process_queue();
            }
        });
    }

    fn process_queue(&mut self) {
        let event = self.event_queue.pop_front().unwrap();
        match event {
            Event::StartEvent(StartEvent {
                name,
                cfg_server_path,
                cfg_tracklist_path,
                resolver,
            }) => {
                let id = self.create_server(name, cfg_server_path, cfg_tracklist_path);
                self.start_server(id);
                let _ = resolver.send(StartEventResponse { id });
            }
            Event::StopEvent(StopEvent { id, resolver }) => {
                self.stop_server(id);
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
        // Opération de recopitage

        let id: u32 = rand::thread_rng().gen::<u32>();
        let serv = Server::new(id, name);
        self.servers.insert(id, serv);
        return id;
    }

    fn start_server(&self, id: u32) {
        let serv = self.servers.get(&id).unwrap();
        serv.start_server();
    }
    fn stop_server(&self, id: u32) {}

    pub fn add_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }
}

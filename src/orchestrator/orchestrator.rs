use crate::events::models::{Event, StartEvent, StopEvent};
use crate::server::server::Server;
use rand::Rng; // 0.8.5
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
        return Self {
            servers: HashMap::new(),
            event_queue: VecDeque::new(),
        };
    }

    pub fn start(&mut self) {
        thread::spawn(|| loop {
            self.process_queue();
        });
    }

    fn process_queue(&mut self) {
        let event = self.event_queue.pop_front().unwrap();
        match event {
            Event::StartEvent(StartEvent {
                name,
                map_path,
                cfg_server_path,
                cfg_tracklist_path,
                resolver,
            }) => {
                let id = self.create_server(name);
                self.start_server(id);
            }
            Event::StopEvent(StopEvent { id, resolver }) => {
                self.stop_server(id);
            }
        }
    }

    fn create_server(&mut self, name: String) -> u32 {
        let id: u32 = rand::thread_rng().gen::<u32>();
        let serv = Server::new(id, name);
        self.servers.insert(id, serv);
        return id;
    }

    fn start_server(&self, id: u32) {}
    fn stop_server(&self, id: u32) {}

    pub fn add_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }
}

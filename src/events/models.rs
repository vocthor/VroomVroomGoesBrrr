use crate::controllers::models::ServerInfo;
use std::path::Path;
use tokio::sync::oneshot::Sender;

pub enum Event {
    StartEvent(StartEvent),
    StopEvent(StopEvent),
    ListEvent(ListEvent),
    GetServerInfoEvent(GetServerInfoEvent),
}

pub struct StartEvent {
    name: String,
    map_path: Box<Path>,
    cfg_server_path: Box<Path>,
    cfg_tracklist_path: Box<Path>,
    resolver: Sender<StartEventResponse>,
}

struct StartEventResponse {
    id: u32,
}

pub struct StopEvent {
    id: u32,
    resolver: Sender<StopEventResponse>,
}
struct StopEventResponse {
    resolver: Sender<StopEventResponse>,
}

struct ListEvent {
    resolver: Sender<ListEventResponse>,
}
struct ListEventResponse {
    servers: Vec<ServerInfo>,
}

struct GetServerInfoEvent {
    id: u32,
    resolver: Sender<GetServerInfoEventResponse>,
}
struct GetServerInfoEventResponse {
    server: ServerInfo,
}

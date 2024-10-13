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
    pub  name: String,
    pub map_path: Box<Path>,
    pub  cfg_server_path: Box<Path>,
    pub cfg_tracklist_path: Box<Path>,
    pub resolver: Sender<StartEventResponse>,
}

pub struct StartEventResponse {
    pub id: u32,
}

pub struct StopEvent {
    pub id: u32,
    pub resolver: Sender<StopEventResponse>,
}
pub struct StopEventResponse {
    pub resolver: Sender<StopEventResponse>,
}

pub struct ListEvent {
    pub resolver: Sender<ListEventResponse>,
}
pub struct ListEventResponse {
    pub servers: Vec<ServerInfo>,
}

pub struct GetServerInfoEvent {
    pub id: u32,
    pub resolver: Sender<GetServerInfoEventResponse>,
}
pub struct GetServerInfoEventResponse {
    pub server: ServerInfo,
}

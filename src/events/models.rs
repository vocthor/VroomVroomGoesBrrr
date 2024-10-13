use std::path::Path;
use tokio::sync::oneshot::Sender;
use crate::controllers::models::ServerInfo;

enum Event {
    StartEvent(StartEvent),
    StopEvent(StopEvent),
    ListEvent(ListEvent),
    GetServerInfoEvent(GetServerInfoEvent),
}

struct StartEvent {
    name: String,
    map_path: Path,
    cfg_server_path: Path,
    cfg_tracklist_path: Path,
    resolver : Sender<StartEventResponse>
}

struct StartEventResponse {
    id: u32,
}

struct StopEvent {
    id: u32,
    resolver : Sender<StopEventResponse>
}
struct StopEventResponse {
    resolver : Sender<StopEventResponse>
}

struct ListEvent {
    resolver : Sender<ListEventResponse>
}
struct ListEventResponse {
    servers: Vec<ServerInfo>,
}

struct GetServerInfoEvent {
    id: u32,
    resolver : Sender<GetServerInfoEventResponse>
}
struct GetServerInfoEventResponse {
    server: ServerInfo,
}
use serde::{Deserialize, Serialize};

static CLI_SOCKET_PATH: &str = "/tmp/vroom-vroom.sock";

#[derive(Serialize, Deserialize, Debug)]
pub enum CliMessage {
    StartServer(StartServerCliMessage),
    StopServer(StopServerCliMessage),
    GetServerInfo(GetServerInfoCliMessage),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct StartServerCliMessage {
    pub name: String,
    pub map_path: String,
    pub cfg_server_path: String,
    pub cfg_tracklist_path: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct StopServerCliMessage {
    pub id : u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetServerInfoCliMessage {
    pub id : u32,
}

pub fn serialize_cli_message(msg: &CliMessage) -> Result<String, serde_json::Error> {
    serde_json::to_string(msg)
}

pub fn deserialize_cli_message(msg: &str) -> Result<CliMessage, serde_json::Error> {
    serde_json::from_str(msg)
}
pub fn get_cli_socket_path() -> &'static str {
    return CLI_SOCKET_PATH;
}
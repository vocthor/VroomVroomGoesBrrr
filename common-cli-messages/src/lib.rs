use serde::{Deserialize, Serialize};

static CLI_MESSAGE_SOCKET_PATH: &str = "/tmp/vroom-vroom-message.sock";
static CLI_RESPONSE_SOCKET_PATH: &str = "/tmp/vroom-vroom-response.sock";

#[derive(Serialize, Deserialize, Debug)]
pub enum CliMessage {
    StartServerCliMessage(StartServerCliMessage),
    StopServerCliMessage(StopServerCliMessage),
    GetServerInfoCliMessage(GetServerInfoCliMessage),
    ListServerCliMessage,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct StartServerCliMessage {
    pub name: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ListServerCliMessage {}

#[derive(Serialize, Deserialize, Debug)]
pub enum CliResponse {
    StartServerCliResponse(StartServerCliResponse),
    StopServerCliResponse(StopServerCliResponse),
    GetServerInfoCliResponse(GetServerInfoCliResponse),
    ListServerCliResponse(ListServerCliResponse),
}



#[derive(Serialize, Deserialize, Debug)]
pub struct StartServerCliResponse {
    pub code: CliResponseCode,
    pub id: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopServerCliResponse {
    pub code: CliResponseCode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetServerInfoCliResponse {
    pub code: CliResponseCode
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListServerCliResponse {
    pub code: CliResponseCode,
    pub servers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CliResponseCode {
    Ok,
    Error,
}

pub fn serialize_cli_message(msg: &CliMessage) -> Result<String, serde_json::Error> {
    serde_json::to_string(msg)
}

pub fn deserialize_cli_message(msg: &str) -> Result<CliMessage, serde_json::Error> {
    serde_json::from_str(msg)
}

pub fn serialize_cli_response(msg: &CliResponse) -> Result<String, serde_json::Error> {
    serde_json::to_string(msg)
}

pub fn deserialize_cli_response(msg: &str) -> Result<CliResponse, serde_json::Error> {
    serde_json::from_str(msg)
}
pub fn get_cli_message_socket_path() -> &'static str {
    return CLI_MESSAGE_SOCKET_PATH;
}
pub fn get_cli_response_socket_path() -> &'static str { return CLI_RESPONSE_SOCKET_PATH; }

pub fn trim_buffer(buf: &[u8]) -> String {
    // Utilise filter pour garder uniquement les caractères valides
    let filtered: String = buf
        .iter()
        .filter(|&&b| b != 0 && b != b'\n') // Retire les caractères nuls et les retours à la ligne
        .map(|&b| b as char) // Convertit les octets en caractères
        .collect(); // Collecte en une String
    filtered
}
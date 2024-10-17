use common_cli_messages::StartServerCliMessage;
use serde::Serialize;
use serde::Deserialize;
use crate::events::models::StartEvent;

#[derive(Serialize)]
pub struct ServerInfo {
    id: u32,
}

impl ServerInfo {
    pub fn new(id: u32) -> Self {
        ServerInfo { id }
    }
}

#[derive(Deserialize)]
pub struct CliCommand{
    message_type : String,
    message : String
}



impl CliCommand {
    pub fn new(message_type: String, message: String) -> Self {
        CliCommand { message_type, message }
    }
    pub fn get_message_type(&self) -> &String {
        &self.message_type
    }
    pub fn get_message(&self) -> &String {
        &self.message
    }
}

#[derive(Deserialize)]
pub struct CreateServerCommand {
    name: String,
    port: u16,
}
#[derive(Deserialize)]
pub struct DeleteServerCommand {
    id: u32,
}
#[derive(Deserialize)]
pub struct GetServerInfoCommand {
    id: u32,
}


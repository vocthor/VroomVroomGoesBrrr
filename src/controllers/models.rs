
use serde::Serialize;
#[derive(Serialize)]
pub struct ServerInfo {
    id: u32,
}

impl ServerInfo {
    pub fn new(id: u32) -> Self {
        ServerInfo { id }
    }
}
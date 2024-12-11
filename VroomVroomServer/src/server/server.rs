use std::{collections::HashMap, hash::Hasher};

use crate::server::docker::DockerComposeCmd;

pub struct Server {
    id: u32,
    name: String,
    port: u16,
    docker_compose_cmd: DockerComposeCmd,
}

impl std::hash::Hash for Server {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
    }
}

impl Eq for Server {}
impl PartialEq for Server {
    fn eq(&self, other: &Server) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl Server {
    pub fn new(id: u32, name: String, port: u16) -> Self {
        let docker_compose_cmd = DockerComposeCmd::new(
            name.as_str(),
            format!("./compose/{}/docker-compose.yaml", id).as_str(),
            "target/docker_logs",
        );
        return Self {
            id,
            name,
            port,
            docker_compose_cmd,
        };
    }

    pub fn start_server(&self) -> bool {
        println!("Server docker-compose up");

        // Setting up the port that will be passed to the docker-compose.yaml file
        let mut env = HashMap::new();
        let port_str: &str = &self.port.to_string();
        env.insert("PORT", port_str);

        return self.docker_compose_cmd.up(env);
    }

    pub fn stop_server(&self) {
        println!("Server docker-compose down");
        self.docker_compose_cmd.down();
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }

    pub fn get_port(&self) -> u16 {
        return self.port;
    }
}

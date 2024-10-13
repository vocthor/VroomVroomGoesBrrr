use bollard::Docker;
use rusty_docker_compose::DockerComposeCmd;

pub struct Server {
    id: u32,
    name: String,
    docker_compose_cmd: DockerComposeCmd,
}

impl Server {
    pub fn new(id: u32, name: String) -> Self {
        let docker_compose_cmd =
            DockerComposeCmd::new("tests/docker-compose.yaml", "target/docker_logs");
        return Self {
            id,
            name,
            docker_compose_cmd,
        };
    }

    pub fn start_server(&self) {
        self.docker_compose_cmd.up();
    }

    pub fn stop_server(&self) {
        self.docker_compose_cmd.down();
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }
}

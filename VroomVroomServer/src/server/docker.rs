use std::{collections::HashMap, process::Command};

#[derive(Clone)]
pub struct DockerComposeCmd {
    name: String,
    file: String,
    logs_dir: String,
}

impl DockerComposeCmd {
    pub fn new(name: &str, file: &str, logs_dir: &str) -> DockerComposeCmd {
        DockerComposeCmd {
            name: name.to_string(),
            file: file.to_string(),
            logs_dir: logs_dir.to_string(),
        }
    }

    pub fn up(&self, env: HashMap<&str, &str>) -> bool {
        let mut cmd_str = String::new();

        // Adding parameters in docker-compose.yaml file
        if !env.is_empty() {
            for ele in env {
                cmd_str += &format!("{}={}", ele.0, ele.1);
            }
        }

        cmd_str += " docker-compose";

        let output = Command::new(cmd_str)
            .arg("-p")
            .arg(self.name.clone())
            .arg("-f")
            .arg(self.file.clone())
            .arg("up")
            .arg("-d")
            .output()
            .expect("Failed to execute up command");

        let success = output.status.success();
        match success {
            // If server correctly starting : nothing particuliar
            true => {
                println!("Output: {}", String::from_utf8_lossy(&output.stdout));
            }
            // If error during startup : stop the server
            false => {
                println!(
                    "An error occured while docker-compose : {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                self.down();
            }
        }
        return success;

        // let dir = &self.logs_dir;
        // if Path::new(dir).exists() {
        //     std::fs::remove_dir_all(dir).unwrap();
        // }
        // std::fs::create_dir_all(dir).unwrap();

        // let output = Command::new("docker-compose")
        //     .arg("-f")
        //     .arg(self.file.clone())
        //     .arg("ps")
        //     .arg("--services")
        //     .output()
        //     .unwrap();

        // let stdout = String::from_utf8(output.stdout).unwrap();
        // let containers: Vec<String> = stdout.lines().map(String::from).collect();

        // let _handles: Vec<_> = containers
        //     .into_iter()
        //     .map(|container| {
        //         let file_name = format!("{}/{}.log", dir, container);
        //         let file_path = std::path::PathBuf::from(file_name);
        //         let docker_compose_file = self.file.clone();
        //         thread::spawn(move || {
        //             let follow_container_log =
        //                 |container: String, file_path: std::path::PathBuf| {
        //                     let file = File::create(file_path).unwrap();
        //                     let _ = Command::new("docker-compose")
        //                         .arg("-f")
        //                         .arg(docker_compose_file)
        //                         .arg("logs")
        //                         .arg("--follow")
        //                         .arg("--no-log-prefix")
        //                         .arg(&container)
        //                         .stdout(Stdio::from(file))
        //                         .spawn()
        //                         .unwrap();
        //                 };

        //             follow_container_log(container, file_path);
        //         });
        //     })
        //     .collect();
    }

    pub fn down(&self) {
        println!("Gracefully shutting down...");

        let _output = Command::new("docker-compose")
            .arg("-p")
            .arg(self.name.clone())
            .arg("-f")
            .arg(self.file.clone())
            .arg("down")
            .output()
            .expect("Failed to execute down command");
    }
}

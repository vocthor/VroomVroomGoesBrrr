use crate::events::models::{
    Event, GetServerInfoEvent, GetServerInfoEventResponse, ListServersEvent,
    ListServersEventResponse, StartEvent, StartEventResponse, StopEvent, StopEventResponse,
};
use common_cli_messages::{
    deserialize_cli_message, get_cli_message_socket_path, get_cli_response_socket_path,
    serialize_cli_response, trim_buffer, CliMessage, CliResponse, CliResponseCode,
    ListServerCliResponse, StartServerCliResponse, StopServerCliResponse,
};
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot::{Receiver, Sender};

pub fn start_cli_controller(queue: Arc<Mutex<VecDeque<Event>>>) {
    tokio::spawn(async move {
        start(queue).await.expect("Failed to start CLI controller");
    });
}

async fn start(queue: Arc<Mutex<VecDeque<Event>>>) -> std::io::Result<()> {
    std::fs::remove_file(get_cli_message_socket_path()).ok();

    // bind to the socket
    let listener =
        UnixListener::bind(get_cli_message_socket_path()).expect("Failed to bind to socket");

    // accept connections and process them
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let queue_clone = Arc::clone(&queue);
                tokio::task::spawn_blocking(move || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async move {
                        handle_client(stream, queue_clone).await;
                    });
                });
            }
            Err(err) => {
                eprintln!("Error accepting connection: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

async fn handle_client(mut stream: UnixStream, queue: Arc<Mutex<VecDeque<Event>>>) {
    let mut buf = [0; 1024];
    println!("Handling client");
    // Read from the stream
    match stream.read(&mut buf) {
        Ok(n) if n > 0 => {
            // Clean the buffer using filter
            let buffered_string = trim_buffer(&buf[..n]);
            println!("Received data: {:?}", buffered_string);
            // Deserialize the JSON
            match deserialize_cli_message(&buffered_string) {
                Ok(message) => {
                    println!("Received message: {:?}", message);
                    match message {
                        CliMessage::StartServerCliMessage(msg) => {
                            println!("Starting server: {:?}", msg);
                            let (resolver, receiver): (
                                Sender<StartEventResponse>,
                                Receiver<StartEventResponse>,
                            ) = tokio::sync::oneshot::channel();
                            let mut event_queue = queue.lock().unwrap();
                            println!("cfg_server_path: {:?}", msg.cfg_server_path);
                            println!("cfg_tracklist_path: {:?}", msg.cfg_tracklist_path);
                            event_queue.push_back(Event::StartEvent(StartEvent {
                                name: msg.name,
                                cfg_server_path: PathBuf::from(msg.cfg_server_path)
                                    .into_boxed_path(),
                                cfg_tracklist_path: PathBuf::from(msg.cfg_tracklist_path)
                                    .into_boxed_path(),
                                resolver: resolver,
                            }));
                            drop(event_queue);

                            let response = receiver.await.expect("Failed to receive response");
                            println!("REPONSE FROM START EVENT : {:?}", response.id);
                            send_response(CliResponse::StartServerCliResponse(
                                StartServerCliResponse {
                                    id: response.id,
                                    code: CliResponseCode::Ok,
                                },
                            ));
                        }
                        CliMessage::StopServerCliMessage(msg) => {
                            let (resolver, receiver): (
                                Sender<StopEventResponse>,
                                Receiver<StopEventResponse>,
                            ) = tokio::sync::oneshot::channel();

                            let mut event_queue = queue.lock().unwrap();
                            event_queue.push_back(Event::StopEvent(StopEvent {
                                id: msg.id,
                                resolver,
                            }));
                            drop(event_queue);

                            let response = receiver.await.expect("Failed to receive response");

                            println!("REPONSE FROM STOP EVENT : {:?}", response);
                            send_response(CliResponse::StopServerCliResponse(
                                StopServerCliResponse {
                                    code: CliResponseCode::Ok,
                                },
                            ));
                        }
                        CliMessage::GetServerInfoCliMessage(msg) => {
                            let (resolver, receiver): (
                                Sender<GetServerInfoEventResponse>,
                                Receiver<GetServerInfoEventResponse>,
                            ) = tokio::sync::oneshot::channel();

                            let mut event_queue = queue.lock().unwrap();
                            event_queue.push_back(Event::GetServerInfoEvent(GetServerInfoEvent {
                                id: msg.id,
                                resolver,
                            }));
                            drop(event_queue);

                            let response = receiver.await.expect("Failed to receive response");
                            send_response(CliResponse::StopServerCliResponse(
                                StopServerCliResponse {
                                    code: CliResponseCode::Ok,
                                },
                            ));
                        }
                        CliMessage::ListServerCliMessage {} => {
                            let (resolver, receiver): (
                                Sender<ListServersEventResponse>,
                                Receiver<ListServersEventResponse>,
                            ) = tokio::sync::oneshot::channel();

                            let mut event_queue = queue.lock().unwrap();
                            event_queue.push_back(Event::ListEvent(ListServersEvent { resolver }));
                            drop(event_queue);

                            let response = receiver.await.expect("Failed to receive response");
                            send_response(CliResponse::ListServerCliResponse(
                                ListServerCliResponse {
                                    code: CliResponseCode::Ok,
                                    servers: vec![],
                                },
                            ));
                        }
                    }
                }
                Err(err) => {
                    println!("Error deserializing JSON: {:?}", err);
                }
            }
        }
        Ok(_) => {
            println!("No data received");
        }
        Err(err) => {
            println!("Error reading from stream: {:?}", err);
        }
    }
}

fn send_response(response: CliResponse) {
    let mut stream = UnixStream::connect(get_cli_response_socket_path());
    match stream {
        Ok(mut stream) => {
            let serialized = serialize_cli_response(&response).unwrap();
            stream.write(serialized.as_bytes()).unwrap();
        }
        Err(err) => {
            eprintln!("Failed to connect to socket: {:?}", err);
        }
    }
}

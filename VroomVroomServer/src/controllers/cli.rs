use crate::events::models::{
    Event, GetServerInfoEvent, GetServerInfoEventResponse, ListServersEvent,
    ListServersEventResponse, StartEvent, StartEventResponse, StopEvent, StopEventResponse,
};
use common_cli_messages::{deserialize_cli_message, get_cli_message_socket_path, serialize_cli_response, trim_buffer, CliMessage, CliResponse, CliResponseCode, GetServerInfoCliMessage, ListServerCliMessage, ListServerCliResponse, StartServerCliMessage, StartServerCliResponse, StopServerCliMessage, StopServerCliResponse};
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

/// Start the CLI controller
///
/// This function will bind to the socket and listen for incoming connections
///
/// # Arguments
///  * `queue` - The event queue
///
/// # Errors
///
/// This function will return an error if it fails to bind to the socket
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

/// Handle the client connection
/// This function will read from the stream and process the message received from the client
///
/// # Arguments
/// * `stream` - The stream to read from
/// * `queue` - The event queue
///
/// # Errors
///
/// This function will return an error if it fails to read from the stream
///
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
                            handle_start_server_message(&queue, msg).await;
                        }
                        CliMessage::StopServerCliMessage(msg) => {
                            handle_stop_server_message(&queue, msg).await;
                        }
                        CliMessage::GetServerInfoCliMessage(msg) => {
                            handle_get_server_info_message(&queue, msg).await;
                        }
                        CliMessage::ListServerCliMessage(msg)=> {
                            handle_list_server_message(&queue, msg).await;
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

/// Handle the list server message
///
/// This function will push a ListServersEvent to the event queue
///
/// # Arguments
/// * `queue` - The event queue
/// * `msg` - The list server message
async fn handle_list_server_message(queue: &Arc<Mutex<VecDeque<Event>>>, msg : ListServerCliMessage) {
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
            error_message: Option::from("".to_string()),
            servers: vec![],
        },
    ),msg.response_socket_path);
}

/// Handle the get server info message
///
/// This function will push a GetServerInfoEvent to the event queue build from the get server info message
///
/// # Arguments
/// * `queue` - The event queue
/// * `msg` - The get server info message
///
async fn handle_get_server_info_message(queue: &Arc<Mutex<VecDeque<Event>>>, msg: GetServerInfoCliMessage) {
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
            error_message: Option::from("".to_string()),
        },
    ),msg.response_socket_path);
}

/// Handle the stop server message
///
/// This function will push a StopEvent to the event queue build from the stop server message
///
/// # Arguments
/// * `queue` - The event queue
/// * `msg` - The stop server message
async fn handle_stop_server_message(queue: &Arc<Mutex<VecDeque<Event>>>, msg: StopServerCliMessage) {
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
            error_message: Option::from("".to_string()),
        },
    ),msg.response_socket_path);
}

/// Handle the start server message
///
/// This function will push a StartEvent to the event queue build from the start server message
///
/// # Arguments
/// * `queue` - The event queue
/// * `msg` - The start server message
async fn handle_start_server_message(queue: &Arc<Mutex<VecDeque<Event>>>, msg: StartServerCliMessage) {
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
            error_message: Option::from("".to_string()),
        },
    ),msg.response_socket_path);
}

/// Send a response to the cli client
///
/// This function will serialize the response and send it to the client
///
/// # Arguments
/// * `response` - The response to send
/// * `socket_path` - The socket path to send the response to
fn send_response(response: CliResponse, socket_path: String) {
    let mut stream = UnixStream::connect(socket_path);
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

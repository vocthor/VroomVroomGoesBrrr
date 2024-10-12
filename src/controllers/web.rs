use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use axum::extract::Path;
use crate::controllers::models::ServerInfo;

pub fn start_web_server(port: u16) {
    tokio::spawn(async move {
        start(port).await;
    });
}

async fn start(port: u16) {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/server/start", post(start_trackmania_server))
        .route("/server/stop", post(stop_trackmania_server))
        .route("/server/list", get(list_trackmania_servers))
        .route("/server/{id}", get(get_trackmania_server_info));


    let addr = format!("localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn start_trackmania_server() -> (StatusCode, Json<&'static str>) {
    (StatusCode::CREATED, Json("Server started"))
}
async fn stop_trackmania_server() -> StatusCode {
    StatusCode::ACCEPTED
}
async fn list_trackmania_servers() -> (StatusCode, Json<Vec<ServerInfo>>) {
    (StatusCode::OK,
     Json(
         vec![
             ServerInfo::new(1),
             ServerInfo::new(2),
             ServerInfo::new(3),
         ]
     )
    )
}
async fn get_trackmania_server_info(Path(id): Path<u32>) -> (StatusCode,Json<ServerInfo>) {
    (StatusCode::OK, Json(ServerInfo::new(id)))
}

async fn hello_handler() -> &'static str {
    "Hello, world!"
}

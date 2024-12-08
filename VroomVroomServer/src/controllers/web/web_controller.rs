use crate::controllers::web::map_controller::{delete_map, get_map_info, list_map, post_map};
use crate::controllers::web::server_config_controller::{
    delete_server_config, get_server_config_info, list_server_config, post_server_config,
};
use crate::controllers::web::server_controller::{
    get_trackmania_server_info, list_trackmania_servers, start_trackmania_server,
    stop_trackmania_server,
};
use crate::controllers::web::tracklist_config_controller::{
    delete_tracklist_config, get_tracklist_config_info, list_tracklist_config,
    post_tracklist_config,
};
use axum::routing::delete;
use axum::{
    routing::{get, post},
    Router,
};

pub fn start_web_controller(port: u16) {
    tokio::spawn(async move {
        start(port).await;
    });
}

async fn start(port: u16) {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        // server related routes
        .route("/server/start", post(start_trackmania_server))
        .route("/server/stop", post(stop_trackmania_server))
        .route("/server/{id}/info", get(get_trackmania_server_info)) // il faudra peut etre utiliser du polling pour celui ci comme la récupération du status d'un serveur est asynchrone
        .route("/server/list", get(list_trackmania_servers))
        // map related routes
        .route("/map", post(post_map))
        .route("/map", get(list_map))
        .route("/map/{id}", delete(delete_map))
        .route("/map/{id}/info", get(get_map_info))
        // server config related routes
        .route("/server-config", post(post_server_config))
        .route("/server-config", get(list_server_config))
        .route("/server-config/{id}", delete(delete_server_config))
        .route("/server-config/{id}/info", get(get_server_config_info))
        // tracklist config related routes
        .route("/tracklist-config", post(post_tracklist_config))
        .route("/tracklist-config", get(list_tracklist_config))
        .route("/tracklist-config/{id}", delete(delete_tracklist_config))
        .route(
            "/tracklist-config/{id}/info",
            get(get_tracklist_config_info),
        );

    let addr = format!("localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

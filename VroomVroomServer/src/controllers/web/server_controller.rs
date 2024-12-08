use crate::controllers::common::models::ServerInfo;
use axum::extract::Path;
use axum::Json;
use http::StatusCode;

pub async fn start_trackmania_server() -> (StatusCode) {

    StatusCode::CREATED
}
pub async fn stop_trackmania_server() -> StatusCode {
    StatusCode::ACCEPTED
}
pub async fn list_trackmania_servers() -> (StatusCode, Json<Vec<ServerInfo>>) {
    (
        StatusCode::OK,
        Json(vec![
            ServerInfo::new(1),
            ServerInfo::new(2),
            ServerInfo::new(3),
        ]),
    )
}
pub async fn get_trackmania_server_info(Path(id): Path<u32>) -> (StatusCode, Json<ServerInfo>) {
    (StatusCode::OK, Json(ServerInfo::new(id)))
}

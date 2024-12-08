
use axum::extract::Path;
use axum::Json;
use http::StatusCode;
use crate::controllers::common::models::ServerInfo;

pub async fn start_trackmania_server() -> (StatusCode, Json<&'static str>) {
    (StatusCode::CREATED, Json("Server started"))
}
pub async fn stop_trackmania_server() -> StatusCode {
    StatusCode::ACCEPTED
}
pub async fn list_trackmania_servers() -> (StatusCode, Json<Vec<ServerInfo>>) {
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
pub async fn get_trackmania_server_info(Path(id): Path<u32>) -> (StatusCode,Json<ServerInfo>) {
    (StatusCode::OK, Json(ServerInfo::new(id)))
}

use axum::extract::Path;
use axum::Json;
use http::StatusCode;
use crate::repository::file_utils::FileInfo;

pub async fn post_tracklist_config() -> StatusCode {
    StatusCode::OK
}

pub async fn list_tracklist_config() -> (StatusCode,Json<Vec<FileInfo>>) {

}

pub async fn delete_tracklist_config(Path(id): Path<u32>) -> StatusCode {
    StatusCode::OK
}

pub async fn get_tracklist_config_info(Path(id): Path<u32>) -> (StatusCode,FileInfo) {
    StatusCode::OK
}
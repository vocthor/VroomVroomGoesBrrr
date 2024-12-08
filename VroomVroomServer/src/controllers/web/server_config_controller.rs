use axum::extract::Path;
use axum::Json;
use http::StatusCode;

use crate::repository::file_utils::FileInfo;

pub async fn post_server_config() -> StatusCode {
    StatusCode::OK
}

pub async fn list_server_config() -> (StatusCode,Json<Vec<FileInfo>>) {

}

pub async fn delete_server_config(Path(id): Path<u32>) -> StatusCode {
    StatusCode::OK
}

pub async fn get_server_config_info(Path(id): Path<u32>) -> (StatusCode,FileInfo) {
    StatusCode::OK
}
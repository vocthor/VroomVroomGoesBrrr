use axum::extract::Path;
use axum::Json;
use http::StatusCode;
use crate::repository::file_utils::FileInfo;

pub async fn post_map() -> StatusCode {
    StatusCode::OK
}

pub async fn list_map() -> (StatusCode,Json<Vec<FileInfo>>) {

}

pub async fn delete_map(Path(id): Path<u32>) -> StatusCode {
    StatusCode::OK
}

pub async fn get_map_info(Path(id): Path<u32>) -> (StatusCode,FileInfo) {
    StatusCode::OK
}


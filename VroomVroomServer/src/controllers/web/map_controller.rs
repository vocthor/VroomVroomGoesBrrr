use crate::repository::file_utils::find_file;
use crate::repository::file_utils::{delete_file, list_file, save_file, FileInfo, FileType};
use axum::extract::{Multipart, Path};
use axum::Json;
use http::StatusCode;

pub async fn post_map(mut multipart: Multipart) -> StatusCode {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("file").to_string();
        let data = field.bytes().await.unwrap();

        match save_file(FileType::Map, file_name, data.to_vec()) {
            Ok(file_info) => {
                println!("Fichier sauvegardé ");
                return StatusCode::OK;
            }
            Err(e) => {
                eprintln!("Erreur lors de la sauvegarde : {:?}", e);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
    }
    StatusCode::BAD_REQUEST
}

pub async fn list_map() -> (StatusCode, Json<Vec<FileInfo>>) {
    (StatusCode::OK, Json(list_file(&FileType::Map)))
}

pub async fn delete_map(Path(id): Path<u32>) -> StatusCode {
    delete_file(&FileType::Map, id);
    StatusCode::OK
}

pub async fn get_map_info(Path(id): Path<u32>) -> (StatusCode, Json<Option<FileInfo>>) {
    if let Some(file_info) = find_file(&FileType::Map, id) {
        (StatusCode::OK, Json(Some(file_info)))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}

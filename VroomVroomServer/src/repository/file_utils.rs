use rand::random;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};
#[derive(Clone, Serialize, Deserialize)]
pub struct FileInfo {
    id: u32,
    name: String,
    file_type: FileType,
}

impl FileInfo {
    fn new(id: u32, file_type: FileType, name: String) -> FileInfo {
        FileInfo {
            id,
            name,
            file_type,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub enum FileType {
    Map,
    ServerConfig,
    TracklistConfig,
}

pub fn save_file(file_type: FileType, name: String, bytes: Vec<u8>) -> Result<FileInfo, Error> {
    let id: u32 = random();
    let file_info = FileInfo::new(id, file_type, name);
    let path = build_file_path(&file_info);

    fs::write(path, bytes).expect("Could not save the file");

    Ok(file_info)
}

pub fn list_file(file_type: &FileType) -> Vec<FileInfo> {
    let folder = get_folder_by_file_type(&file_type);
    let mut result: Vec<FileInfo> = Vec::new();

    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(filename) = path.file_stem().and_then(|f| f.to_str()) {
                if let Some((id, name)) = parse_file_name_without_extension(filename) {
                    result.push(FileInfo::new(id, file_type.clone(), name))
                }
            }
        }
    }
    result
}

pub fn find_file(file_type: &FileType, id: u32) -> Option<FileInfo> {
    let folder = get_folder_by_file_type(&file_type);

    if let Ok(entries) = fs::read_dir(folder) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(filename) = path.file_stem().and_then(|f| f.to_str()) {
                if let Some((file_id, name)) = parse_file_name_without_extension(filename) {
                    if id == file_id {
                        return Some(FileInfo::new(id, file_type.clone(), name));
                    }
                }
            }
        }
    }
    None
}

pub fn delete_file(file_type: &FileType, id: u32) {
    if let Some(file_info) = find_file(file_type, id) {
        let path = build_file_path(&file_info);
        fs::remove_file(path).expect("Cannot delete file")
    }
}

fn get_folder_by_file_type(file_type: &FileType) -> String {
    create_folder_if_not_exist("/tmp/VroomVroom/map/").expect("Cannot create folder");
    create_folder_if_not_exist("/tmp/VroomVroom/serverconfig/").expect("Cannot create folder");
    create_folder_if_not_exist("/tmp/VroomVroom/tracklistconfig/").expect("Cannot create folder");
    match file_type {
        FileType::Map => String::from("/tmp/VroomVroom/map/"),
        FileType::ServerConfig => String::from("/tmp/VroomVroom/serverconfig/"),
        FileType::TracklistConfig => String::from("/tmp/VroomVroom/tracklistconfig/"),
    }
}

fn create_folder_if_not_exist(path: &str) -> std::io::Result<()> {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    Ok(())
}

fn get_file_extension_by_file_type(file_type: &FileType) -> String {
    match file_type {
        FileType::Map => String::from(".Gbx"),
        FileType::ServerConfig => String::from(".xml"),
        FileType::TracklistConfig => String::from(".xml"),
    }
}

fn build_file_path(file_info: &FileInfo) -> PathBuf {
    let mut path_string = get_folder_by_file_type(&file_info.file_type);
    path_string.push_str(file_info.id.to_string().as_str());
    path_string.push_str("---");
    path_string.push_str(file_info.name.as_str());
    path_string.push_str(get_file_extension_by_file_type(&file_info.file_type).as_str());
    PathBuf::from(path_string)
}

fn parse_file_name_without_extension(file_name: &str) -> Option<(u32, String)> {
    let parts: Vec<&str> = file_name.split("---").collect();
    if let [id_str, name] = parts[..] {
        if let Ok(id) = str::parse::<u32>(id_str) {
            return Some((id, String::from(name)));
        }
    }
    None
}

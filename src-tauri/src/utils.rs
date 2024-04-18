use std::fs;
use std::path::{Path, PathBuf};

pub async fn check_or_create(path: &str) -> () {
    if !Path::new(path).exists() {
        fs::create_dir_all(path).expect("failed to create directory");
    }
}

pub enum FileType {
    Directory,
    File,
    All,
}

pub fn get_directory_content(dir: &Path, scan_type: &FileType) -> Vec<PathBuf> {
    let entries = fs::read_dir(dir).expect("failed to read directory");

    let mut dir_list: Vec<PathBuf> = Vec::new();

    for entry in entries {
        let path = entry.expect("failed to get entry").path();

        let new_dir = dir.join(path.file_name().unwrap());

        match scan_type {
            FileType::Directory => {
                if path.is_dir() {
                    dir_list.push(new_dir);
                }
            }
            FileType::File => {
                if path.is_file() {
                    dir_list.push(new_dir);
                }
            }
            FileType::All => {
                dir_list.push(new_dir);
            }
        }
    }

    return dir_list;
}

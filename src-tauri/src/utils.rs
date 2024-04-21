use std::fs;
use std::path::{Path, PathBuf};

use rand::random;
use walkdir::WalkDir;

pub async fn check_or_create(path: &str) {
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
    WalkDir::new(dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| match scan_type {
            FileType::Directory => e.path().is_dir(),
            FileType::File => e.path().is_file(),
            FileType::All => true,
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

pub fn get_random_file(dir: &Path) -> Option<PathBuf> {
    
    let mut current_dir = dir.to_path_buf();
    loop {
        let label_dirs = get_directory_content(&current_dir, &FileType::Directory);

        if label_dirs.is_empty() {
            break;
        }

        let random_number = random::<usize>() % label_dirs.len();
        let random_dir = label_dirs.get(random_number).unwrap();

        current_dir = random_dir.clone();
    }


    let file_dirs = get_directory_content(&current_dir, &FileType::File);

    if file_dirs.is_empty() {
        return None;
    }

    let random_number = random::<usize>() % file_dirs.len();

    Option::from(file_dirs.get(random_number).unwrap().to_path_buf())
}

pub fn remove_directory_content(dir: &Path) {
    fs::remove_dir_all(dir).unwrap_or(());
}

// pub fn emit_event(event: &str, payload: &str) -> () {
//     println!("{}: {}", event, payload);
//     let window = window
// }

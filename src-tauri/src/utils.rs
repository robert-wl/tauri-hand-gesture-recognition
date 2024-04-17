use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

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

pub fn get_directory_content(dir: &str, scan_type: &FileType) -> Vec<String> {
    let entries = fs::read_dir(dir).expect("failed to read directory");

    let mut dir_list: Vec<String> = Vec::new();

    for entry in entries {
        let path = entry.expect("failed to get entry").path();

        let dir_name = path.file_name().unwrap().to_str().unwrap();

        let new_dir = format!("{}/{}", dir, dir_name);

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


pub fn create_python_venv() {
    println!("Creating python venv function called");

    let project_root = std::env::current_dir().unwrap();

    println!("Project root: {:?}", project_root);
    println!("Joined path: {:?}", project_root.join("scripts\\create_venv.bat"));
    let out = Command::new(project_root.join(project_root.join("scripts\\create_venv.bat")))
        .spawn()
        .expect("failed to create python venv");
    
    println!("Output: {:?}", out.stdout);
    // if !Path::new(".venv").exists() {
    //     println!("Creating python venv");
    //     Command::new("python3.11")
    //         .arg("-m")
    //         .arg("venv")
    //         .arg(".venv")
    //         .output()
    //         .expect("failed to create python venv");
    //
    //     println!("Installing requirements");
    //     let requirements_txt = File::open("/scripts/requirements.txt")
    //         .expect("failed to open requirements.txt");
    //
    //     println!("Reading requirements");
    //     let requirements = BufReader::new(requirements_txt);
    //
    //     println!("Installing requirements");
    //     for req in requirements.lines() {
    //         println!("Installing requirement: {:?}", req);
    //         let req = req.unwrap();
    //         Command::new(".venv/Scripts/pip")
    //             .arg("install")
    //             .arg(req)
    //             .output()
    //             .expect("failed to install requirements");
    //     }
    // }
}
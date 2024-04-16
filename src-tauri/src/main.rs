// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::process::Command;

use taurpc::Router;

use crate::dataset::api::DatasetApi;
use crate::dataset::api_impl::DatasetApiImpl;
use crate::util::api::UtilApi;
use crate::util::api_impl::UtilApiImpl;

mod dataset {
    pub mod api;
    pub mod api_impl;
    pub mod dataset;
}


mod util {
    pub mod api;
    pub mod api_impl;
}


fn create_python_venv() -> Result<(), String> {
    let output = Command::new("python")
        .arg("-m")
        .arg("venv")
        .arg(".venv")
        .output()
        .expect("failed to create python venv");

    println!("Status: {}", output.status);
    Ok(())
}


#[tokio::main]
async fn main() {
    let router = Router::new()
        .merge(DatasetApiImpl.into_handler())
        .merge(UtilApiImpl.into_handler());

    tauri::Builder::default()
        .invoke_handler(router.into_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

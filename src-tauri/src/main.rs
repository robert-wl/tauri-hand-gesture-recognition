// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::process::Command;


use taurpc::Router;

use crate::dataset::api::DatasetApi;
use crate::dataset::api_impl::DatasetApiImpl;
use crate::util::api::UtilApi;
use crate::util::api_impl::UtilApiImpl;
use crate::utils::create_python_venv;

pub mod dataset;
pub mod util;
pub mod utils;


#[tokio::main]
async fn main() {
    let _ = create_python_venv();

    let router = Router::new()
        .merge(DatasetApiImpl.into_handler())
        .merge(UtilApiImpl.into_handler());

    tauri::Builder::default()
        .invoke_handler(router.into_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

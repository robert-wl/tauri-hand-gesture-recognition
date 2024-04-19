// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use taurpc::Router;

use crate::dataset::api::{DatasetApi, TauRpcDatasetApiEventTrigger};
use crate::dataset::api_impl::DatasetApiImpl;
use crate::util::api::UtilApi;
use crate::util::api_impl::UtilApiImpl;

pub mod dataset;
mod py_utils;
pub mod util;
pub mod utils;

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

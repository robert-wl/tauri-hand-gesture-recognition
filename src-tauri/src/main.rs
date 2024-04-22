// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use taurpc::Router;

use crate::dataset::api::DatasetApi;
use crate::dataset::api_impl::DatasetApiImpl;
use crate::model::api::ModelApi;
use crate::model::api_impl::ModelApiImpl;
use crate::util::api::UtilApi;
use crate::util::api_impl::UtilApiImpl;

mod constants;
mod dataset;
mod model;
mod py_utils;
mod util;
mod utils;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .merge(DatasetApiImpl.into_handler())
        .merge(UtilApiImpl.into_handler())
        .merge(ModelApiImpl.into_handler());

    tauri::Builder::default()
        .invoke_handler(router.into_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

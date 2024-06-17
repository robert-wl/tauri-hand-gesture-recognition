use std::collections::HashMap;

use crate::dataset::dataset::{Dataset, GeneralDataset, Label, TestingDataset, TrainingDataset};

#[taurpc::procedures(path = "dataset")]
pub trait DatasetApi {
    async fn get_all() -> Result<Vec<GeneralDataset>, String>;
    async fn get_all_training_dataset() -> Result<Vec<TrainingDataset>, String>;
    async fn get_all_testing_dataset() -> Result<Vec<TestingDataset>, String>;
    async fn get_random_image(path: String) -> Result<String, String>;
    async fn get_random_processed_image(path: String) -> Result<String, String>;
    async fn get_labels(dataset_name: String) -> Result<Vec<Label>, String>;
    async fn get_data(dataset_name: String, label_name: String) -> Result<Vec<String>, String>;
    async fn get(dataset_name: String) -> Result<Dataset, String>;
    async fn preprocess(dataset_name: String, app_handle: tauri::AppHandle) -> Result<(), String>;
    async fn get_image(name: String, label: String, data: String) -> Result<String, String>;
    async fn get_processed_image(
        name: String,
        label: String,
        data: String,
    ) -> Result<Option<String>, String>;
    async fn get_processed_graphs(name: String) -> Result<HashMap<String, String>, String>;
}

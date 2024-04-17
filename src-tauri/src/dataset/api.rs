use crate::dataset::dataset::{Dataset, DatasetLabel};

#[taurpc::procedures(path = "dataset")]
pub trait DatasetApi {
    async fn get_datasets() -> Result<Vec<Dataset>, String>;
    async fn get_dataset_thumbnail(dataset_name: String) -> Result<String, String>;
    async fn get_dataset_label_data(dataset_name: String) -> Result<Vec<DatasetLabel>, String>;
    async fn get_dataset(dataset_name: String) -> Result<Dataset, String>;
    async fn preprocess_dataset(dataset_name: String) -> Result<(), String>;
}
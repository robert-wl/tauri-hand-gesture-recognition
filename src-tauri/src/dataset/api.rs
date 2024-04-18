use crate::dataset::dataset::{Dataset, DatasetData, DatasetLabel};

#[taurpc::procedures(path = "dataset")]
pub trait DatasetApi {
    async fn get_all() -> Result<Vec<Dataset>, String>;
    async fn get_random_thumbnail(dataset_name: String) -> Result<String, String>;
    async fn get_labels(dataset_name: String) -> Result<Vec<DatasetLabel>, String>;
    async fn get_data(dataset_name: String, label_name: String)
        -> Result<Vec<DatasetData>, String>;
    async fn get(dataset_name: String) -> Result<Dataset, String>;
    async fn preprocess(dataset_name: String) -> Result<(), String>;
    async fn get_thumbnail(name: String, label: String, data: String) -> Result<String, String>;
    async fn get_processed_thumbnail(
        name: String,
        label: String,
        data: String,
    ) -> Result<String, String>;
}

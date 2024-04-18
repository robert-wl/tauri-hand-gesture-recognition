use crate::dataset::dataset::{Dataset, GeneralDataset, Label};

#[taurpc::procedures(path = "dataset")]
pub trait DatasetApi {
    async fn get_all() -> Result<Vec<GeneralDataset>, String>;
    async fn get_random_thumbnail(path: String) -> Result<String, String>;
    async fn get_labels(dataset_name: String) -> Result<Vec<Label>, String>;
    async fn get_data(dataset_name: String, label_name: String)
        -> Result<Vec<String>, String>;
    async fn get(dataset_name: String) -> Result<Dataset, String>;
    async fn preprocess(dataset_name: String) -> Result<(), String>;
    async fn get_thumbnail(name: String, label: String, data: String) -> Result<String, String>;
    async fn get_processed_thumbnail(
        name: String,
        label: String,
        data: String,
    ) -> Result<Option<String>, String>;
}

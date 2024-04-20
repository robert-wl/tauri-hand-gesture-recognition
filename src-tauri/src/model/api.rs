use crate::dataset::dataset::{Dataset, GeneralDataset, Label};

#[taurpc::procedures(path = "model")]
pub trait ModelApi {
    async fn train(dataset_name: String, model_name: String, kernel: String) -> Result<(), String>;
}

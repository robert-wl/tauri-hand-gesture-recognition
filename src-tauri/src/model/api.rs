use crate::dataset::dataset::{Dataset, GeneralDataset, Label};
use crate::model::model::{Model, ModelHyperparameter};

#[taurpc::procedures(path = "model")]
pub trait ModelApi {
    async fn train(
        dataset_name: String,
        model_name: String,
        hyperparameter: ModelHyperparameter,
    ) -> Result<(), String>;
    async fn get(model_name: String) -> Result<Model, String>;
    async fn remove(model_name: String) -> Result<(), String>;
}

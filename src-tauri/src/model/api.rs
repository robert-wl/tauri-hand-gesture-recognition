use crate::model::model::{Model, ModelHyperparameter, ModelPrediction};

#[taurpc::procedures(path = "model")]
pub trait ModelApi {
    async fn train(
        dataset_name: String,
        model_name: String,
        hyperparameter: ModelHyperparameter,
    ) -> Result<(), String>;
    async fn get_all() -> Result<Vec<Model>, String>;
    async fn get(model_name: String) -> Result<Model, String>;
    async fn remove(model_name: String) -> Result<(), String>;
    async fn predict(model_name: String, image: String) -> Result<ModelPrediction, String>;
}

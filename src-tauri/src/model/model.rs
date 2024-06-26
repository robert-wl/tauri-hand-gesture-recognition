use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use specta::Type;

#[taurpc::ipc_type]
pub struct Model {
    pub name: String,
    pub model_specification: ModelSpecification,
    pub confusion_matrix_image: String,
}
#[taurpc::ipc_type]
pub struct SVMHyperparameter {
    pub kernel: String,
    #[serde(rename = "C")]
    pub c: String,
    pub gamma: String,
    pub degree: String,
}

#[taurpc::ipc_type]
pub struct KNNHyperparameter {
    pub n_neighbors: String,
    pub algorithm: String,
    pub weights: String,
    pub metric: String,
}

#[taurpc::ipc_type]
pub struct LRHyperparameter {
    pub penalty: String,
    #[serde(rename = "C")]
    pub c: String,
    pub solver: String,
    pub max_iter: String,
}



#[taurpc::ipc_type]
pub struct ModelSpecification {
    pub dataset_name: String,
    pub accuracy: f32,
    pub precision: f32,
    pub recall: f32,
    pub f1: f32,
    pub confusion_matrix: Vec<Vec<u16>>,
    pub hyperparameters: Hyperparameters,
    pub classification_report: ClassificationReport,
}

#[taurpc::ipc_type]
pub struct ClassificationReport {
    pub class: HashMap<String, Metrics>,
    pub accuracy: f32,
    pub macro_avg: Metrics,
    pub weighted_avg: Metrics,
}

#[taurpc::ipc_type]
pub struct Metrics {
    pub precision: f32,
    pub recall: f32,
    #[serde(rename = "f1-score")]
    pub f1: f32,
    pub support: f32,
}

#[taurpc::ipc_type]
pub struct ModelPrediction {
    pub class: String,
    pub image_result: String,
}

#[derive(Clone, Serialize, Deserialize, Type)]
pub enum Hyperparameters {
    Svm(SVMHyperparameter),
    Knn(KNNHyperparameter),
    Lr(LRHyperparameter),
}

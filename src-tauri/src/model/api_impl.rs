use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use base64::Engine;
use base64::engine::general_purpose;

use crate::constants::{
    CONFUSION_MATRIX_IMAGE, MODEL_SCRIPT, MODEL_SPECIFICATION_JSON, MODELS_DIRECTORY,
    PROCESSED_DIRECTORY, PROCESSED_OUTPUT_CSV, SCRIPTS_DIRECTORY,
};
use crate::model::api::ModelApi;
use crate::model::model::{Model, ModelHyperparameter, ModelSpecification};
use crate::py_utils::run_script;

#[derive(Clone)]
pub struct ModelApiImpl;

#[taurpc::resolvers]
impl ModelApi for ModelApiImpl {
    async fn train(
        self,
        dataset_name: String,
        model_name: String,
        hyperparameter: ModelHyperparameter,
    ) -> Result<(), String> {
        let csv = Path::new(PROCESSED_DIRECTORY)
            .join(dataset_name)
            .join(PROCESSED_OUTPUT_CSV);

        let file = File::open(&csv);

        if file.is_err() {
            return Err("Dataset not found".to_string());
        }
        println!("{:?}", file);

        let dataset_path = csv.to_str().unwrap().to_string();

        let script_path = Path::new(SCRIPTS_DIRECTORY).join(MODEL_SCRIPT);

        let mut child = run_script(
            &script_path,
            vec![
                dataset_path,
                model_name,
                hyperparameter.kernel,
                hyperparameter.c,
                hyperparameter.gamma,
                hyperparameter.degree,
            ],
        );

        let stderr = child.stderr.take().unwrap();
        let reader = BufReader::new(stderr);

        println!("Training model...");
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }

        let status = child.wait().expect("failed to wait on child");

        if !status.success() {
            return Err("Failed to train model".to_string());
        }

        Ok(())
    }

    async fn get(self, model_name: String) -> Result<Model, String> {
        let model_dir = Path::new(MODELS_DIRECTORY).join(&model_name);

        let evaluation_json = model_dir.join(MODEL_SPECIFICATION_JSON);

        let mut file = File::open(evaluation_json).expect("Failed to open evaluation.json");
        let mut data = String::new();

        file.read_to_string(&mut data)
            .expect("Failed to read evaluation.json");

        let model: ModelSpecification =
            serde_json::from_str(&data).expect("Failed to parse evaluation.json");

        let confusion_matrix_image = model_dir.join(CONFUSION_MATRIX_IMAGE);

        let image = fs::read(confusion_matrix_image).expect("failed to read thumbnail");

        let base64_image = general_purpose::STANDARD.encode(image);

        let model = Model {
            name: model_name,
            model_specification: model,
            confusion_matrix_image: base64_image,
        };

        Ok(model)
    }

    async fn remove(self, model_name: String) -> Result<(), String> {
        let model_dir = Path::new(MODELS_DIRECTORY).join(&model_name);

        fs::remove_dir_all(model_dir).unwrap_or(());

        Ok(())
    }
}

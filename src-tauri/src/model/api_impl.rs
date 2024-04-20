use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::constants::{
    MODEL_SCRIPT, PROCESSED_DIRECTORY, PROCESSED_OUTPUT_CSV, SCRIPTS_DIRECTORY,
};
use crate::model::api::ModelApi;
use crate::py_utils::run_script;

#[derive(Clone)]
pub struct ModelApiImpl;

#[taurpc::resolvers]
impl ModelApi for ModelApiImpl {
    async fn train(
        self,
        dataset_name: String,
        model_name: String,
        kernel: String,
    ) -> Result<(), String> {
        let csv = Path::new(PROCESSED_DIRECTORY)
            .join(dataset_name)
            .join(PROCESSED_OUTPUT_CSV);

        let file = File::open(&csv);

        if file.is_err() {
            return Err("Dataset not found".to_string());
        }
        
        let dataset_path = csv.to_str().unwrap().to_string();

        let script_path = Path::new(SCRIPTS_DIRECTORY).join(MODEL_SCRIPT);

        let mut child = run_script(&script_path, vec![dataset_path, model_name, kernel]);

        let stderr = child.stderr.take().unwrap();
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            println!("{}", line.unwrap());
        }

        let status = child.wait().expect("failed to wait on child");

        if !status.success() {
            return Err("Failed to train model".to_string());
        }
        
        Ok(())
    }
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use futures::future;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use tauri::Manager;

use crate::constants::{CONVERTER_SCRIPT, DATASET_DIRECTORY, GRAPH_SCRIPT, MODEL_DIRECTORY, MODEL_SPECIFICATION_JSON, PROCESSED_DIRECTORY, PROCESSED_OUTPUT_CSV, PROCESSED_OUTPUT_LDA_GRAPH, PROCESSED_OUTPUT_PCA_GRAPH, PROCESSED_OUTPUT_TSNE_GRAPH, SCRIPTS_DIRECTORY};
use crate::dataset::api::DatasetApi;
use crate::dataset::dataset::{Dataset, GeneralDataset, Label, ProgressPayload, TestingDataset, TrainingDataset};
use crate::model::model::ModelSpecification;
use crate::py_utils::run_script;
use crate::utils::{FileType, get_directory_content, get_random_file, read_file, remove_directory_content};

#[derive(Clone)]
pub struct DatasetApiImpl;

#[taurpc::resolvers]
impl DatasetApi for DatasetApiImpl {
    async fn get_all(self) -> Result<Vec<GeneralDataset>, String> {
        let dataset_dirs =
            get_directory_content(Path::new(DATASET_DIRECTORY), &FileType::Directory);

        let datasets = dataset_dirs
            .par_iter()
            .map(|dir| {
                let name = dir
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap();

                let mut dataset = GeneralDataset {
                    name,
                    label_amount: 0,
                    data_amount: 0,
                };

                let label_dir = Path::new(&dir);

                let label_dirs = get_directory_content(label_dir, &FileType::Directory);


                dataset.label_amount = label_dirs.len() as u32;
                dataset.data_amount = label_dirs
                    .par_iter()
                    .map(|dir| get_directory_content(dir.as_path(), &FileType::File).len() as u32)
                    .sum();

                dataset
            })
            .collect();

        Ok(datasets)
    }

    async fn get_all_training_dataset(self) -> Result<Vec<TrainingDataset>, String> {
        let processed_dir = Path::new(PROCESSED_DIRECTORY);

        let dataset_dirs = get_directory_content(processed_dir, &FileType::Directory);

        let model_datasets: Vec<TrainingDataset> = dataset_dirs
            .par_iter()
            .filter_map(|dataset| {
                let csv = dataset.join(PROCESSED_OUTPUT_CSV);
                let file = File::open(csv);

                if file.is_err() {
                    return None;
                }

                let name = dataset
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap();

                let mut rdt = csv::Reader::from_reader(file.unwrap());

                let data_amount = rdt.records().count() as u32;
                let feature_count = (rdt.headers().unwrap().len() - 2) as u32;

                let model_dataset = TrainingDataset {
                    name,
                    data_amount,
                    feature_count,
                };

                Some(model_dataset)
            })
            .collect();

        Ok(model_datasets)
    }

    async fn get_all_testing_dataset(self) -> Result<Vec<TestingDataset>, String> {
        let model_dir = Path::new(MODEL_DIRECTORY);

        let model_dirs = get_directory_content(model_dir, &FileType::Directory);

        let testing_datasets: Vec<TestingDataset> = model_dirs
            .par_iter()
            .map(|dir| {
                let json = dir.join(MODEL_SPECIFICATION_JSON);

                let mut file = File::open(json).expect("Failed to open specification.json");
                let mut data = String::new();

                file.read_to_string(&mut data)
                    .expect("Failed to read evaluation.json");

                let model: ModelSpecification =
                    serde_json::from_str(&data).expect("Failed to parse specification.json");

                let name = dir
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap();

                TestingDataset {
                    name,
                    dataset_name: model.dataset_name,
                    accuracy: model.accuracy,
                }
            })
            .collect();

        Ok(testing_datasets)
    }

    async fn get(self, name: String) -> Result<Dataset, String> {
        let labels = self.clone().get_labels(name.clone()).await.unwrap();

        let dataset = Dataset {
            name: name.to_string(),
            labels,
        };

        Ok(dataset)
    }

    async fn get_labels(self, name: String) -> Result<Vec<Label>, String> {
        let dataset_dir = Path::new(DATASET_DIRECTORY).join(&name);

        let dir_list = get_directory_content(&dataset_dir, &FileType::Directory);


        let tasks: Vec<_> = dir_list
            .iter()
            .map(|dir| {
                let name = name.clone();
                let self_clone = self.clone();
                let dir = dir.clone();
                tokio::spawn(async move {
                    let dataset_label = dir
                        .file_name()
                        .unwrap()
                        .to_os_string()
                        .into_string()
                        .unwrap();

                    let data = self_clone
                        .clone()
                        .get_data(name.clone(), dataset_label.clone())
                        .await
                        .unwrap();

                    let is_preprocessed = self_clone
                        .get_processed_image(name.clone(), dataset_label.clone(), data[0].clone())
                        .await
                        .unwrap()
                        .is_some();

                    Label {
                        name: dataset_label.clone(),
                        data,
                        is_preprocessed,
                    }
                })
            })
            .collect::<Vec<_>>();

        let data_label = future::join_all(tasks)
            .await
            .into_iter()
            .filter_map(|task| task.ok())
            .collect();

        Ok(data_label)
    }

    async fn get_data(self, name: String, label: String) -> Result<Vec<String>, String> {
        let label_dir = Path::new(DATASET_DIRECTORY).join(name).join(label);

        let file_dirs = get_directory_content(&label_dir, &FileType::File);

        let data = file_dirs
            .par_iter()
            .map(|file| {
                file.file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap()
            })
            .collect();

        Ok(data)
    }

    async fn get_random_image(self, path: String) -> Result<String, String> {
        let dataset_dir = Path::new(DATASET_DIRECTORY).join(path);

        let random_file = get_random_file(&dataset_dir);

        if random_file.is_none() {
            return Err("Failed to get random image".to_string());
        }

        let base64_thumbnail = match read_file(&random_file.unwrap()) {
            Some(base64_thumbnail) => base64_thumbnail,
            None => return Err("Failed to read random image".to_string()),
        };

        Ok(base64_thumbnail)
    }

    async fn get_random_processed_image(self, path: String) -> Result<String, String> {
        let processed_dir = Path::new(PROCESSED_DIRECTORY).join(path);

        let random_file = get_random_file(&processed_dir);

        if random_file.is_none() {
            return Err("Failed to get random processed image".to_string());
        }

        let base64_thumbnail = match read_file(&random_file.unwrap()) {
            Some(base64_thumbnail) => base64_thumbnail,
            None => return Err("Failed to read random processed image".to_string()),
        };

        Ok(base64_thumbnail)
    }

    async fn get_image(self, name: String, label: String, data: String) -> Result<String, String> {
        let file_dir = Path::new(DATASET_DIRECTORY)
            .join(name)
            .join(label)
            .join(data);

        let base64_thumbnail = match read_file(&file_dir) {
            Some(base64_thumbnail) => base64_thumbnail,
            None => return Err("Failed to read image".to_string()),
        };

        Ok(base64_thumbnail)
    }

    async fn get_processed_image(
        self,
        name: String,
        label: String,
        data: String,
    ) -> Result<Option<String>, String> {
        let dataset_dir = Path::new(PROCESSED_DIRECTORY)
            .join(name)
            .join(label)
            .join(data);

        let base64_thumbnail = read_file(&dataset_dir);

        Ok(base64_thumbnail)
    }

    async fn preprocess(self, name: String, app_handle: tauri::AppHandle) -> Result<(), String> {
        let in_path = Path::new(DATASET_DIRECTORY).join(&name);
        let out_path = Path::new(PROCESSED_DIRECTORY).join(&name);

        remove_directory_content(&out_path);

        let label_dirs = get_directory_content(&in_path, &FileType::Directory);
        let out_csv_str = out_path
            .join(PROCESSED_OUTPUT_CSV)
            .to_str()
            .unwrap()
            .to_string();


        for dir in label_dirs {
            let data_count = get_directory_content(&dir, &FileType::File).len() ;
            let label = dir.file_name().unwrap().to_str().unwrap().to_string();
            let in_dir_str = dir.to_str().unwrap().to_string();
            let out_dir_str = out_path.join(&label).to_str().unwrap().to_string();

            let script_path = Path::new(SCRIPTS_DIRECTORY).join(CONVERTER_SCRIPT);

            let mut child = run_script(&script_path, vec![in_dir_str, out_dir_str, out_csv_str.clone()]);

            let stdout = child.stdout.take().unwrap();
            let reader = BufReader::new(stdout);

            for (current_amount, _) in reader.lines().enumerate() {
                let payload = ProgressPayload {
                    name: name.clone(),
                    label: label.clone(),
                    current_amount: (current_amount + 1) as u32,
                    total_amount: data_count as u32,
                };

                println!("progress_{}", current_amount);
                app_handle
                    .emit_all(format!("progress_{}", label).as_str(), payload)
                    .expect("failed to emit event");
            }


            let stderr = child.stderr.take().unwrap();
            let reader = BufReader::new(stderr);

            for line in reader.lines() {
                println!("{}", line.unwrap());
            }

            let status = child.wait().expect("failed to wait on child");

            if !status.success() {
                return Err("Failed to preprocess dataset".to_string());
            }
        }


        let script_path = Path::new(SCRIPTS_DIRECTORY).join(GRAPH_SCRIPT);

        let out_path_str = out_path.to_str().unwrap().to_string();


        let mut child = run_script(&script_path, vec![out_csv_str, out_path_str]);

        let stderr = child.stderr.take().unwrap();
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            println!("{}", line.unwrap());
        }

        let status = child.wait().expect("failed to wait on child");

        if !status.success() {
            return Err("Failed to preprocess dataset".to_string());
        }


        Ok(())
    }

    async fn get_processed_graphs(self, name: String) -> Result<HashMap<String, String>, String> {
        let processed_dir = Path::new(PROCESSED_DIRECTORY).join(name);

        let graph_names = vec![PROCESSED_OUTPUT_LDA_GRAPH, PROCESSED_OUTPUT_PCA_GRAPH, PROCESSED_OUTPUT_TSNE_GRAPH];
        
        let mut graphs: HashMap<String, String> = HashMap::new();

        for graph in graph_names {
            let graph_path = processed_dir.join(graph);
            let base64_thumbnail = match read_file(&graph_path) {
                Some(base64_thumbnail) => base64_thumbnail,
                None => return Err("Failed to read graph".to_string()),
            };
            
            let graph_str = graph.to_string();

            graphs.insert(graph_str, base64_thumbnail);
        }

        Ok(graphs)
    }
}

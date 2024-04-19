use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

use base64::Engine;
use base64::engine::general_purpose;
use rand::random;
use tauri::Manager;

use crate::dataset::api::DatasetApi;
use crate::dataset::dataset::{Dataset, GeneralDataset, Label, ProgressPayload};
use crate::py_utils::run_script;
use crate::utils::{FileType, get_directory_content, remove_directory_content};

#[derive(Clone)]
pub struct DatasetApiImpl;

const DIRECTORY: &str = "dataset";
const PROCESSED_DIRECTORY: &str = "processed";
const CONVERTER_SCRIPT: &str = "mediapipe_converter.py";

#[taurpc::resolvers]
impl DatasetApi for DatasetApiImpl {
    async fn get_all(self) -> Result<Vec<GeneralDataset>, String> {
        let mut datasets = Vec::new();

        let dataset_dirs = get_directory_content(Path::new(DIRECTORY), &FileType::Directory);

        for dir in dataset_dirs {
            let name = dir
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();

            let mut dataset = GeneralDataset {
                name: name.clone(),
                label_amount: 0,
                data_amount: 0,
            };

            let label_dir = Path::new(&dir);

            let label_dirs = get_directory_content(label_dir, &FileType::Directory);

            dataset.label_amount = label_dirs.len() as u16;
            for dir in label_dirs {
                dataset.data_amount += get_directory_content(&dir, &FileType::File).len() as u16;
            }

            datasets.push(dataset);
        }

        Ok(datasets)
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
        let dataset_dir = Path::new(DIRECTORY).join(name.clone());

        let dir_list = get_directory_content(&dataset_dir, &FileType::Directory);

        let mut data_label: Vec<Label> = Vec::new();

        for dir in dir_list {
            let dataset_label = dir
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();

            let data = self
                .clone()
                .get_data(name.clone(), dataset_label.clone())
                .await
                .unwrap();

            let is_preprocessed = self
                .clone()
                .get_processed_image(name.clone(), dataset_label.clone(), data[0].clone())
                .await
                .unwrap()
                .is_some();

            let dataset_label = Label {
                name: dataset_label,
                data,
                is_preprocessed,
            };

            data_label.push(dataset_label);
        }

        Ok(data_label)
    }

    async fn get_data(self, name: String, label: String) -> Result<Vec<String>, String> {
        let label_dir = Path::new(DIRECTORY).join(name.clone()).join(label.clone());

        let file_dirs = get_directory_content(&label_dir, &FileType::File);

        let data = file_dirs
            .iter()
            .map(|file| {
                let file_name = file
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap();

                file_name
            })
            .collect();

        Ok(data)
    }

    async fn get_random_image(self, path: String) -> Result<String, String> {
        let dataset_dir = Path::new(DIRECTORY).join(path);

        let mut current_dir = dataset_dir.clone();
        loop {
            let label_dirs = get_directory_content(&current_dir, &FileType::Directory);

            if label_dirs.is_empty() {
                break;
            }

            let random_number = random::<usize>() % label_dirs.len();
            let random_dir = label_dirs.get(random_number).unwrap();

            current_dir = random_dir.clone();
        }

        let file_dirs = get_directory_content(&current_dir, &FileType::File);

        let random_number = random::<usize>() % file_dirs.len();
        let random_file = file_dirs.get(random_number).unwrap();

        let thumbnail = fs::read(random_file).unwrap_or_default();

        let base64_thumbnail = general_purpose::STANDARD.encode(thumbnail);
        Ok(base64_thumbnail)
    }

    async fn get_image(self, name: String, label: String, data: String) -> Result<String, String> {
        let file_dir = Path::new(DIRECTORY).join(name).join(label).join(data);

        let thumbnail = fs::read(file_dir).expect("failed to read thumbnail");

        let base64_thumbnail = general_purpose::STANDARD.encode(thumbnail);

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

        let thumbnail = fs::read(dataset_dir).unwrap_or_default();

        if thumbnail.is_empty() {
            return Ok(None);
        }

        let base64_thumbnail = general_purpose::STANDARD.encode(thumbnail);

        Ok(Option::from(base64_thumbnail))
    }

    async fn preprocess(self, name: String, app_handle: tauri::AppHandle) -> Result<(), String> {
        let in_path = Path::new(DIRECTORY).join(name.clone());
        let out_path = Path::new(PROCESSED_DIRECTORY).join(name.clone());

        remove_directory_content(&out_path);

        let label_dirs = get_directory_content(&in_path, &FileType::Directory);

        for dir in label_dirs {
            let data_count = get_directory_content(&dir, &FileType::File).len() as u16;
            let label = dir
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let in_dir_str = dir
                .to_str()
                .unwrap()
                .to_string();
            let out_dir_str = out_path
                .join(label.clone())
                .to_str()
                .unwrap()
                .to_string();

            let script_path = Path::new("scripts").join(CONVERTER_SCRIPT);

            let mut child = run_script(&script_path, vec![in_dir_str, out_dir_str]);

            let stdout = child.stdout.take().unwrap();
            let reader = BufReader::new(stdout);

            for (current_amount, _) in reader.lines().enumerate() {
                let payload = ProgressPayload {
                    name: name.clone(),
                    label: label.clone(),
                    current_amount: (current_amount + 1) as u16,
                    total_amount: data_count,
                };

                app_handle
                    .emit_all(format!("progress_{}", label).as_str(), payload)
                    .expect("failed to emit event");
            }

            let stderr = child.stderr.take().unwrap();
            let reader = BufReader::new(stderr);

            for line in reader.lines() {
                println!("{}", line.unwrap());
            }

            println!("Waiting for child to finish");

            let status = child.wait().expect("failed to wait on child");

            if !status.success() {
                return Err("Failed to preprocess dataset".to_string());
            }
        }

        Ok(())
    }
}

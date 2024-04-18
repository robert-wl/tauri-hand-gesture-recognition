use std::env::current_dir;
use base64::Engine;
use std::fs;
use std::path::Path;

use crate::dataset::api::DatasetApi;
use crate::dataset::dataset::{Dataset, GeneralDataset, Label};
use crate::utils::{get_directory_content, FileType};
use base64::engine::general_purpose;
use rand::random;
use crate::dataset::api::TauRpcDatasetApiInputs::get_data;

#[derive(Clone)]
pub struct DatasetApiImpl;

const DIRECTORY: &str = "dataset";
const PROCESSED_DIRECTORY: &str = "processed";

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
        let labels = self.clone()
            .get_labels(name.clone())
            .await
            .unwrap();

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


            let dataset_label = Label {
                name: dataset_label,
                data
            };

            data_label.push(dataset_label);
        }

        Ok(data_label)
    }

    async fn get_data(self, name: String, label: String) -> Result<Vec<String>, String> {
        let label_dir = Path::new(DIRECTORY).join(name.clone()).join(label.clone());

        let file_dirs = get_directory_content(&label_dir, &FileType::File);

        let data = file_dirs.iter()
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

    async fn get_random_thumbnail(self, path: String) -> Result<String, String> {
        let dataset_dir = Path::new(DIRECTORY).join(path);

        let mut current_dir = dataset_dir.clone();
        while true {
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

    async fn get_thumbnail(
        self,
        name: String,
        label: String,
        data: String,
    ) -> Result<String, String> {
        let file_dir = Path::new(DIRECTORY).join(name).join(label).join(data);

        let thumbnail = fs::read(file_dir).expect("failed to read thumbnail");

        let base64_thumbnail = general_purpose::STANDARD.encode(thumbnail);

        Ok(base64_thumbnail)
    }

    async fn get_processed_thumbnail(
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

    async fn preprocess(self, dataset_name: String) -> Result<(), String> {
        let in_dir = Path::new(DIRECTORY).join(dataset_name.clone());
        let out_dir = Path::new(PROCESSED_DIRECTORY).join(dataset_name.clone());

        let dir_list = get_directory_content(&in_dir, &FileType::Directory);

        // for dir in dir_list {
        //     println!("dir: {:?}", dir);
        //     let dir_name = dir.split("/").last().unwrap();
        //     let fin_in_dir = format!("{}\\{}", in_dir, dir_name);
        //     let fin_out_dir = format!("{}\\{}", out_dir, dir_name);
        //
        //     let script_path = "scripts\\mediapipe_converter.py";
        //
        //     let status = run_script(script_path, vec![fin_in_dir, fin_out_dir]);
        //
        //     if !status.success() {
        //         return Err("Failed to preprocess dataset".to_string());
        //     }
        // }

        Ok(())
    }

    // async fn preprocess_dataset(self, dataset_name: String, label: String) -> Result<(), String> {
    //     let in_dir = format!("{}\\{}", DIRECTORY, dataset_name);
    //     let out_dir = format!("processed\\{}", dataset_name);
    //
    //     let dir_list = get_directory_content(&in_dir, &FileType::Directory);
    //
    //     for dir in dir_list {
    //         println!("dir: {:?}", dir);
    //         let dir_name = dir.split("/").last().unwrap();
    //         let fin_in_dir = format!("{}\\{}", in_dir, dir_name);
    //         let fin_out_dir = format!("{}\\{}", out_dir, dir_name);
    //
    //         let script_path = "scripts\\mediapipe_converter.py";
    //
    //         let status = run_script(script_path, vec![fin_in_dir, fin_out_dir]);
    //
    //         if !status.success() {
    //             return Err("Failed to preprocess dataset".to_string());
    //         }
    //     }
    //
    //     Ok(())
    // }
}

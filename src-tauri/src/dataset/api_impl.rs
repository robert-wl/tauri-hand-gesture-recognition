use base64::Engine;
use std::fs;
use std::path::Path;

use crate::dataset::api::DatasetApi;
use crate::dataset::dataset::{Dataset, DatasetData, DatasetLabel};
use crate::utils::{get_directory_content, FileType};
use base64::engine::general_purpose;
use rand::random;

#[derive(Clone)]
pub struct DatasetApiImpl;

const DIRECTORY: &str = "dataset";
const PROCESSED_DIRECTORY: &str = "processed";

#[taurpc::resolvers]
impl DatasetApi for DatasetApiImpl {
    async fn get_all(self) -> Result<Vec<Dataset>, String> {
        let mut datasets = Vec::new();

        let dataset_dirs = get_directory_content(Path::new(DIRECTORY), &FileType::Directory);

        for dir in dataset_dirs {
            let name = dir
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();

            let mut dataset = Dataset {
                name: name.clone(),
                label_amount: 0,
                data_amount: 0,
                thumbnail: String::new(),
            };

            let label_dir = Path::new(&dir);

            let label_dirs = get_directory_content(label_dir, &FileType::Directory);

            dataset.label_amount = label_dirs.len() as u16;
            for dir in label_dirs {
                dataset.data_amount += get_directory_content(&dir, &FileType::File).len() as u16;
            }

            dataset.thumbnail = self
                .clone()
                .get_random_thumbnail(name.to_string())
                .await
                .unwrap();

            datasets.push(dataset);
        }

        Ok(datasets)
    }

    async fn get(self, name: String) -> Result<Dataset, String> {
        let dataset_dir = Path::new(DIRECTORY).join(name.clone());

        let dir_list = get_directory_content(&dataset_dir, &FileType::Directory);

        let mut dataset = Dataset {
            name: name.to_string(),
            label_amount: 0,
            data_amount: 0,
            thumbnail: String::new(),
        };

        dataset.label_amount = dir_list.len() as u16;

        dataset.data_amount = dir_list.iter()
            .map(|dir| get_directory_content(&dir, &FileType::File).len() as u16)
            .sum();

        dataset.thumbnail = self
            .clone()
            .get_random_thumbnail(name.to_string())
            .await
            .unwrap();

        Ok(dataset)
    }

    async fn get_labels(self, name: String) -> Result<Vec<DatasetLabel>, String> {
        let dataset_dir = Path::new(DIRECTORY).join(name.clone());

        let dir_list = get_directory_content(&dataset_dir, &FileType::Directory);

        let data_label: Vec<DatasetLabel> = dir_list.iter()
            .map(|dir| {
                let dataset_label = dir
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap();

                let mut dataset_label = DatasetLabel {
                    name: dataset_label,
                    data_amount: 0,
                    thumbnail: String::new(),
                };

                let file_dirs = get_directory_content(&dir, &FileType::File);

                dataset_label.data_amount = file_dirs.len() as u16;

                let random_number = random::<usize>() % file_dirs.len();

                let random_file = file_dirs.get(random_number).unwrap();

                let thumbnail = fs::read(random_file).expect("failed to read thumbnail");

                dataset_label.thumbnail = general_purpose::STANDARD.encode(thumbnail);

                dataset_label
            })
            .collect();

        Ok(data_label)
    }

    async fn get_data(self, name: String, label: String) -> Result<Vec<DatasetData>, String> {
        let label_dir = Path::new(DIRECTORY).join(name.clone()).join(label.clone());

        let file_dirs = get_directory_content(&label_dir, &FileType::File);

        let mut data: Vec<DatasetData> = Vec::new();

        for file in file_dirs {
            let file_name = file
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();

            let raw_thumbnail = self
                .clone()
                .get_thumbnail(name.clone(), label.clone(), file_name.clone())
                .await
                .unwrap();

            let processed_thumbnail = self
                .clone()
                .get_processed_thumbnail(name.clone(), label.clone(), file_name.clone())
                .await
                .unwrap();

            let dataset_data: DatasetData = DatasetData {
                name: file_name,
                raw_thumbnail,
                processed_thumbnail,
            };

            data.push(dataset_data);
        }

        Ok(data)
    }

    async fn get_random_thumbnail(self, name: String) -> Result<String, String> {
        let dataset_dir = Path::new(DIRECTORY).join(name.clone());

        let label_dirs = get_directory_content(&dataset_dir, &FileType::Directory);

        let random_number = random::<usize>() % label_dirs.len();
        let random_dir = label_dirs.get(random_number).unwrap();

        let file_dirs = get_directory_content(random_dir, &FileType::File);

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
        let dataset_dir = Path::new(DIRECTORY).join(name).join(label).join(data);

        let dir_list = get_directory_content(&dataset_dir, &FileType::Directory);

        let random_number = random::<usize>() % dir_list.len();
        let random_dir = dir_list.get(random_number).unwrap();

        let file_list = get_directory_content(random_dir, &FileType::File);

        let random_number = random::<usize>() % file_list.len();
        let random_file = file_list.get(random_number).unwrap();

        let thumbnail = fs::read(random_file).expect("failed to read thumbnail");

        let base64_thumbnail = general_purpose::STANDARD.encode(thumbnail);
        
        Ok(base64_thumbnail)
    }

    async fn get_processed_thumbnail(
        self,
        name: String,
        label: String,
        data: String,
    ) -> Result<String, String> {
        let dataset_dir = Path::new(PROCESSED_DIRECTORY)
            .join(name)
            .join(label)
            .join(data);

        let dir_list = get_directory_content(&dataset_dir, &FileType::Directory);

        let random_number = random::<usize>() % dir_list.len();
        let random_dir = dir_list.get(random_number).unwrap();

        let file_list = get_directory_content(random_dir, &FileType::File);

        let random_number = random::<usize>() % file_list.len();
        let random_file = file_list.get(random_number).unwrap();

        let thumbnail = fs::read(random_file).expect("failed to read thumbnail");

        let base64_thumbnail = general_purpose::STANDARD.encode(thumbnail);
        
        Ok(base64_thumbnail)
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

use std::fs;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};

use rand::random;
use crate::dataset::api::DatasetApi;
use crate::dataset::dataset::{Dataset, DatasetLabel};
use crate::py_utils::run_script;
use crate::utils::{FileType, get_directory_content};

#[derive(Clone)]
pub struct DatasetApiImpl;

const directory : &str = "dataset";

#[taurpc::resolvers]
impl DatasetApi for DatasetApiImpl {
    async fn get_datasets(self) -> Result<Vec<Dataset>, String> {

        let mut datasets = Vec::new();

        let dir_list = get_directory_content(directory, &FileType::Directory);

        for dir in dir_list {
            let dataset_name = dir.split("/").last().unwrap();
            let mut dataset = Dataset {
                name: dataset_name.to_string(),
                label_amount: 0,
                data_amount: 0,
                thumbnail: String::new(),
            };

            let dataset_type = format!("{}/{}", directory, dataset_name);

            println!("dataset_type: {:?}", dataset_type);
            let type_dir_list = get_directory_content(&dataset_type, &FileType::Directory);

            for type_dir in type_dir_list {
                dataset.label_amount += 1;
                for _ in get_directory_content(&type_dir, &FileType::File) {
                    dataset.data_amount += 1;
                }
            }

            dataset.thumbnail = self.clone().get_dataset_thumbnail(dataset_name.to_string()).await.unwrap();

            datasets.push(dataset);
        }

        Ok(datasets)
    }

    async fn get_dataset(self, dataset_name: String) -> Result<Dataset, String> {
        let dataset_dir = format!("{}/{}", directory, dataset_name);

        let dir_list = get_directory_content(&dataset_dir, &FileType::Directory);

        let mut dataset = Dataset {
            name: dataset_name.to_string(),
            label_amount: 0,
            data_amount: 0,
            thumbnail: String::new(),
        };

        for dir in dir_list {
            dataset.label_amount += 1;
            for _ in get_directory_content(&dir, &FileType::File) {
                dataset.data_amount += 1;
            }
        }

        dataset.thumbnail = self.clone().get_dataset_thumbnail(dataset_name.to_string()).await.unwrap();

        Ok(dataset)
    }

    async fn get_dataset_label_data(self, dataset_name: String) -> Result<Vec<DatasetLabel>, String> {
        let dataset_dir = format!("{}/{}", directory, dataset_name);

        let dir_list = get_directory_content(&dataset_dir, &FileType::Directory);

        let mut data_label: Vec<DatasetLabel> = Vec::new();

        for dir in dir_list {

            let mut dataset_label = DatasetLabel {
                name: dir.split("/").last().unwrap().to_string(),
                data_amount: 0,
                thumbnail: String::new(),
            };

            let file_list = get_directory_content(&dir, &FileType::File);

            dataset_label.data_amount = file_list.len() as u16;

            let random_number = random::<usize>() % file_list.len();

            let random_file = file_list.get(random_number).unwrap();

            let thumbnail = fs::read(random_file).expect("failed to read thumbnail");

            dataset_label.thumbnail = base64::encode(thumbnail);

            data_label.push(dataset_label);
        }

        Ok(data_label)
    }



    async fn get_dataset_thumbnail(self, dataset_name: String) -> Result<String, String> {
        let dataset_dir = format!("{}/{}", directory, dataset_name);

        let dir_list = get_directory_content(&dataset_dir, &FileType::Directory);

        let random_number = random::<usize>() % dir_list.len();
        let random_dir = dir_list.get(random_number).unwrap();

        let file_list = get_directory_content(random_dir, &FileType::File);

        let random_number = random::<usize>() % file_list.len();
        let random_file = file_list.get(random_number).unwrap();

        let thumbnail = fs::read(random_file).expect("failed to read thumbnail");

        let base64_thumbnail = base64::encode(thumbnail);
        Ok(base64_thumbnail)
    }


    async fn preprocess_dataset(self, dataset_name: String) -> Result<(), String> {
        let in_dir = format!("{}\\{}", directory, dataset_name);
        let out_dir = format!("processed\\{}", dataset_name);

        let dir_list = get_directory_content(&in_dir, &FileType::Directory);

        for dir in dir_list {
            println!("dir: {:?}", dir);
            let dir_name = dir.split("/").last().unwrap();
            let fin_in_dir = format!("{}\\{}", in_dir, dir_name);
            let fin_out_dir = format!("{}\\{}", out_dir, dir_name);

            let script_path = "scripts\\mediapipe_converter.py";

            let status = run_script(script_path, vec![fin_in_dir, fin_out_dir]);

            if !status.success() {
                return Err("Failed to preprocess dataset".to_string());
            }
        }

        Ok(())
    }
}






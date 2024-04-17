use std::fs;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};

use rand::random;
use crate::dataset::api::DatasetApi;
use crate::dataset::dataset::Dataset;
use crate::py_utils::run_script;
use crate::utils::{FileType, get_directory_content};

#[derive(Clone)]
pub struct DatasetApiImpl;


#[taurpc::resolvers]
impl DatasetApi for DatasetApiImpl {
    async fn get_datasets(self) -> Result<Vec<Dataset>, String> {
        let directory = "dataset";

        let mut datasets = Vec::new();

        let dir_list = get_directory_content(directory, &FileType::Directory);

        for dir in dir_list {
            let dataset_name = dir.split("/").last().unwrap();
            let mut dataset = Dataset {
                name: dataset_name.to_string(),
                data_amount: 0,
                thumbnail: String::new(),
            };

            let dataset_type = format!("{}/{}", directory, dataset_name);

            println!("dataset_type: {:?}", dataset_type);
            let type_dir_list = get_directory_content(&dataset_type, &FileType::Directory);

            for type_dir in type_dir_list {
                for _ in get_directory_content(&type_dir, &FileType::File) {
                    dataset.data_amount += 1;
                }
            }

            dataset.thumbnail = self.clone().get_dataset_thumbnail(dataset_name.to_string()).await.unwrap();

            datasets.push(dataset);
        }

        Ok(datasets)
    }

    async fn get_dataset_thumbnail(self, dataset_name: String) -> Result<String, String> {
        let directory = format!("dataset/{}", dataset_name);

        let dir_list = get_directory_content(&directory, &FileType::Directory);

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
        let in_dir = format!("dataset\\{}", dataset_name);
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






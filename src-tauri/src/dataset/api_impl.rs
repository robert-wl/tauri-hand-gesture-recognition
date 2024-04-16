use std::fs;

use crate::dataset::api::DatasetApi;
use crate::dataset::dataset::Dataset;

#[derive(Clone)]
pub struct DatasetApiImpl;


#[taurpc::resolvers]
impl DatasetApi for DatasetApiImpl {
    async fn get_datasets(self) -> Result<Vec<Dataset>, String> {
        let directory = "dataset";

        let mut datasets = Vec::new();

        check_or_create(directory);

        let dir_list = get_directory_content(directory, &FileType::Directory);

        for dir in dir_list {
            let dataset_name = dir.split("/").last().unwrap();
            let mut dataset = Dataset {
                name: dataset_name.to_string(),
                data_amount: 0,
            };

            let dataset_type = format!("{}/{}", directory, dataset_name);

            let type_dir_list = get_directory_content(&dataset_type, &FileType::Directory);

            for type_dir in type_dir_list {
                for _ in get_directory_content(&type_dir, &FileType::File) {
                    dataset.data_amount += 1;
                }
            }

            datasets.push(dataset);
        }

        Ok(datasets)
    }

    async fn get_dataset_thumbnail(self, dataset_name: String) -> Result<Vec<u8>, String> {
        let directory = format!("dataset/{}", dataset_name);

        let dir_list = get_directory_content(&directory, &FileType::File);

        let thumbnail_path = format!("{}/thumbnail.png", directory);

        let thumbnail = fs::read(thumbnail_path).expect("failed to read thumbnail");

        Ok(thumbnail)
    }
}


enum FileType {
    Directory,
    File,
    All,
}


fn check_or_create(path: &str) {
    if !std::path::Path::new(path).exists() {
        fs::create_dir_all(path).expect("failed to create directory");
    }
}

fn get_directory_content(dir: &str, scan_type: &FileType) -> Vec<String> {
    let entries = fs::read_dir(dir).expect("failed to read directory");

    let mut dir_list: Vec<String> = Vec::new();

    for entry in entries {
        let path = entry.expect("failed to get entry").path();

        let dir_name = path.file_name().unwrap().to_str().unwrap();

        let new_dir = format!("{}/{}", dir, dir_name);

        match scan_type {
            FileType::Directory => {
                if path.is_dir() {
                    dir_list.push(new_dir);
                }
            }
            FileType::File => {
                if path.is_file() {
                    dir_list.push(new_dir);
                }
            }
            FileType::All => {
                dir_list.push(new_dir);
            }
        }
    }

    return dir_list;
}
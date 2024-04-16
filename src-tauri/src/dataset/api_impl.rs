use std::fs;

use rand::random;
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

    async fn open_dataset_directory(self) -> Result<(), String> {
        let directory = "dataset";

        check_or_create(directory);

        let _ = std::process::Command::new("explorer")
            .arg(directory)
            .output()
            .expect("failed to open directory");

        Ok(())
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
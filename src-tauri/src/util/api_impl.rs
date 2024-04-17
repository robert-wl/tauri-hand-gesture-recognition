use crate::util::api::UtilApi;
use crate::utils::check_or_create;

#[derive(Clone)]
pub struct UtilApiImpl;


#[taurpc::resolvers]
impl UtilApi for UtilApiImpl {
    async fn get_current_dir(self) -> Result<String, String> {
        let exe_path = std::env::current_exe();

        match exe_path {
            Ok(path) => {
                let path_str = path.to_str().unwrap();
                let path_str = path_str.rsplitn(2, "/").last().unwrap();
                Ok(path_str.to_string())
            }
            Err(_) => Err("failed to get current directory".to_string())
        }
    }

    async fn open_directory(self) -> Result<(), String> {
        let directory = "dataset";

        check_or_create(directory).await;

        let _ = std::process::Command::new("explorer")
            .arg(directory)
            .output()
            .expect("failed to open directory");

        Ok(())
    }
}


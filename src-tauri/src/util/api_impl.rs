use crate::util::api::UtilApi;

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
}


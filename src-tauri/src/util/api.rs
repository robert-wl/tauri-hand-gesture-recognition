use std::path::PathBuf;

#[taurpc::procedures(path = "util")]
pub trait UtilApi {
    async fn get_current_dir() -> Result<String, String>;
    async fn open_directory() -> Result<(), String>;
}

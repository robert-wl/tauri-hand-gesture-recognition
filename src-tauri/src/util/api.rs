#[taurpc::procedures(path = "util")]
pub trait UtilApi {
    async fn get_current_dir() -> Result<String, String>;
}
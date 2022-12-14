pub trait Generator {
    fn generate(&self) -> Result<String, serde_json::error::Error>;
}
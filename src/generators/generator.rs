pub trait Generator {
    fn generate(&self) -> Result<String, serde_json::error::Error>;

    fn serialize(&self) -> Result<String, serde_json::error::Error>;

    fn deserialize(&self, json: &str) -> Result<Box::<Self>, serde_json::error::Error>;
}
pub trait Generator {
    fn generate_includes(&self) -> String {
        "".to_string()
    }

    fn generate_globals(&self) -> String {
        "".to_string()
    }
    
    fn generate_init(&self) -> String {
        "".to_string()
    }

    fn generate_loop_one_time_setup(&self) -> String {
        "".to_string()
    }
    
    fn generate_loop(&self) -> String {
        "".to_string()
    }

    fn serialize(&self) -> Result<String, serde_json::error::Error>;

    fn deserialize(&self, json: &str) -> Result<Box::<Self>, serde_json::error::Error>;
    
    fn render_options(&mut self, ui: &mut egui::Ui, id: usize);
}

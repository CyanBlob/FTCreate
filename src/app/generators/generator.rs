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

    fn serialize(&self) -> Result<String, serde_json::error::Error> where Self: serde::Serialize {
        serde_json::to_string_pretty(self)
    }

    fn deserialize<'a>(&self, json: &'a str) -> Result<Box::<Self>, serde_json::error::Error> where Self: serde::Deserialize<'a> {
        match serde_json::from_str::<Self>(json) {
            Ok(s) => {
                Ok(Box::new(s))
            }
            Err(e) => {
                Err(e)
            }
            
        }
    }
    
    fn render_options(&mut self, ui: &mut egui::Ui, id: usize);
}

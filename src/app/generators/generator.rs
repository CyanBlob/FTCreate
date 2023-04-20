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

    fn render_options(&mut self, ui: &mut egui::Ui, id: usize);
}

pub trait GeneratorSerialize: serde::Serialize + Generator {}

pub trait SubsystemGenerator {
    fn get_name(&self) -> String {
        "Subsystem".to_string()
    }
}

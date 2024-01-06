use crate::app::generators::{
    generator::{self, SubsystemGenerator},
};
use strum_macros::EnumIter;
use crate::app::generators::lua_generator::{ControlHandler, LuaGenerator};

#[derive(
Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, EnumIter, PartialOrd, Copy,
)]
pub enum DrivetrainType {
    Mecanum,
    Arcade,
    Tank,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Subsystem
{
    pub name: String,
    pub control_handler: ControlHandler,
    pub selected_control: String,
}

impl
generator::Generator for Subsystem
{
    fn generate_includes(&self) -> String {
        let mut code: String = "".to_owned();

        code += &*self.control_handler.generate_includes().to_string();

        code
    }

    fn generate_globals(&self) -> String {
        let mut code = "".to_string();

        code += &*self.control_handler.generate_globals().to_string();

        code
    }

    fn generate_init(&self) -> String {
        let mut code = "".to_string();

        code += &*self.control_handler.generate_init().to_string();

        code
    }

    fn generate_loop_one_time_setup(&self) -> String {
        let mut code: String = "".to_owned();

        code += &*self.control_handler.generate_loop_one_time_setup().to_string();

        code
    }

    fn generate_loop(&self) -> String {
        let mut code = "".to_string();

        code += &*self.control_handler.generate_loop().to_string();

        code
    }

    fn render_options(&mut self, ui: &mut egui::Ui, _id: usize) {
        egui::scroll_area::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.add_space(20.0);

                egui::ComboBox::new(format!("{}.{}", self.name, "Controls"), "Controls List")
                    .selected_text(format!("{}", &mut self.selected_control))
                    .width(170.0)
                    .show_ui(ui, |ui| {
                        for script in &self.control_handler.scripts {
                            let script_name = script.split("/").last().unwrap().split(".").nth(0).unwrap();
                            ui.selectable_value(
                                &mut self.selected_control,
                                script.clone(),
                                format!("{:?}", script_name),
                            );
                        }
                    });

                if ui.button("Add selected control").clicked() {
                    let mut new_generator = LuaGenerator::new(&self.selected_control);
                    new_generator.load();
                    self.control_handler.generators.push(new_generator);
                }

                self.control_handler.render(ui);

                ui.add_space(30.0);
            });
    }
}

impl
SubsystemGenerator for Subsystem
{
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl Subsystem
{
    pub fn new(name: String) -> Self {
        Subsystem {
            name: name.to_string(),
            control_handler: ControlHandler { scripts: vec![], generators: vec![] },
            selected_control: "".to_string(),
        }
    }
}

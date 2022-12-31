use serde::{Deserialize, Serialize};
use serde_json::Result;

use strum::IntoEnumIterator;

use super::motor;
use super::super::generator;

use crate::app::generators::motors;

use motor::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct CoreHD {
    pub direction: motors::motor::MotorDirection,
    pub mode: motors::motor::MotorMode,
    pub max_speed: f64,
    pub position: u8,
}

impl generator::Generator for CoreHD {
    fn generate(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
    }

    fn serialize(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
    }

    fn deserialize(&self, json: &str) -> Result<Box::<Self>> {
        match serde_json::from_str::<Self>(json) {
            Ok(s) => {
                Ok(Box::new(s))
            }
            Err(e) => {
                Err(e)
            }
            
        }
    }

    fn render_options(&mut self, ui: &mut egui::Ui, id: usize) {

            let max_speed = 1.0;
            
            ui.label("Core HD");
            ui.add_space(10.0);

            ui.push_id(id, |ui| {
                egui::ComboBox::from_label("Run mode")
                    .selected_text(format!("{:?}", &mut self.mode))
                    .width(150.0) 
                    .show_ui(ui, |ui| {
                    for mode in motor::MotorMode::iter() {
                        ui.selectable_value(&mut self.mode, mode, format!("{:?}", mode));
                    }
                });
            });

            ui.push_id(id, |ui| {
                egui::ComboBox::from_label("Direction")
                    .selected_text(format!("{:?}", &mut self.direction))
                    .width(150.0) 
                    .show_ui(ui, |ui| {
                    for direction in motor::MotorDirection::iter() {
                        ui.selectable_value(&mut self.direction, direction, format!("{:?}", direction));
                    }
                });
            });

            ui.add_space(20.0);

            ui.add(egui::Slider::new(&mut self.max_speed, -max_speed..=max_speed).text("Max speed"));

            if ui.button("Increment").clicked() {
                self.max_speed += 0.1;

                if self.max_speed > max_speed {
                    self.max_speed = max_speed;
                }
            }

            if ui.button("Decrement").clicked() {
                self.max_speed -= 0.1;

                if self.max_speed < -max_speed {
                    self.max_speed = -max_speed;
                }
            }
    }
}

impl Motor for CoreHD {}

impl MotorGenerator for CoreHD {}

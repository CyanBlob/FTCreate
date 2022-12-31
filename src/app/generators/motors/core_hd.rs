use serde::{Deserialize, Serialize};
use serde_json::Result;

use super::motor;
use super::super::generator;

use crate::app::generators::motors;

use motor::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct CoreHD {
    pub direction: motors::motor::MotorDirection,
    pub max_speed: f64,
    pub position: u8,
    pub reversed: bool
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

    fn render_options(&mut self, ui: &mut egui::Ui) {

            let max_speed = 1.0;
            
            ui.label("Core HD");
            ui.add(egui::Slider::new(&mut self.max_speed, -max_speed..=max_speed).text("Max speed"));
            ui.checkbox(&mut self.reversed, "Reversed?");

            if self.reversed {
                self.direction = motors::motor::MotorDirection::REVERSE;
            } 
            else {
                self.direction = motors::motor::MotorDirection::FORWARD;
            }
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

use std::ops::RangeInclusive;
use egui::Ui;
use crate::app::generators::control::UiElement;
use crate::app::generators::keybinding::keybinding::Keybinding;

#[derive(Clone, Debug)]
pub struct Slider {
    pub name: String,
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub step_by: f64,
    pub deicimals: usize,
    pub label: String,
    pub keybinding: Option<Keybinding<f32>>,
}

#[derive(Clone, Debug)]
pub struct TextInput {
    pub name: String,
    pub value: String,
    pub label: String,
}

impl UiElement for Slider {
    fn render(&mut self, ui: &mut Ui) {
        ui.add(
            egui::Slider::new(&mut self.value, RangeInclusive::new(self.min, self.max))
                .text(&self.label)
                .max_decimals(self.deicimals)
                .step_by(self.step_by)
            ,
        );
    }
}

impl UiElement for TextInput {
    fn render(&mut self, ui: &mut Ui) {
        ui.text_edit_singleline(&mut self.value);
    }
}
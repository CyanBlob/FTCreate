use std::ops::RangeInclusive;
use egui::Ui;
use crate::app::generators::control::UiElement;
use crate::app::generators::keybinding::keybinding::Keybinding;

#[derive(Clone)]
pub struct Slider {
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub step_by: f32,
    pub deicimals: usize,
    pub label: String,
    pub keybinding: Option<Keybinding<f32>>,
}

impl UiElement for Slider {
    fn render(&mut self, ui: &mut Ui) {
        ui.add(
            egui::Slider::new(&mut self.value, RangeInclusive::new(self.min, self.max))
                .text(&self.label)
                .max_decimals(self.deicimals),
        );
    }
}
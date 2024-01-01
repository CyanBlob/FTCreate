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

#[derive(Clone, Debug)]
pub struct ComboBoxInput {
    pub name: String,
    pub value: String,
    pub label: String,
    pub entries: Vec::<String>,
    pub id: i32,
}

#[derive(Clone, Debug)]
pub struct CheckboxInput {
    pub name: String,
    pub label: String,
    pub value: bool,
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

impl UiElement for ComboBoxInput {
    fn render(&mut self, ui: &mut Ui) {
        egui::ComboBox::new(format!("{}{}{}", &self.name, &self.label, &self.id), &self.label)
            .selected_text(format!("{:?}", &mut self.value))
            .width(170.0)
            .show_ui(ui, |ui| {
                for entry in &self.entries {
                    ui.selectable_value(&mut self.value, entry.to_string(), format!("{:?}", entry));
                }
            });
    }
}
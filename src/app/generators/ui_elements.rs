use crate::app::generators::control::UiElement;
use crate::app::generators::keybinding::keybinding::Keybinding;
use egui::Ui;
use mlua::{Function, Lua};
use std::ops::RangeInclusive;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TextInput {
    pub name: String,
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ComboBoxInput {
    pub name: String,
    pub value: String,
    pub label: String,
    pub entries: Vec<String>,
    pub id: i32,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CheckboxInput {
    pub name: String,
    pub label: String,
    pub value: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ButtonInput {
    pub name: String,
    pub callback: String,
}

impl UiElement for Slider {
    fn render(&mut self, ui: &mut Ui, _lua: Option<&Lua>) {
        ui.add(
            egui::Slider::new(&mut self.value, RangeInclusive::new(self.min, self.max))
                .text(&self.label)
                .max_decimals(self.deicimals)
                .step_by(self.step_by),
        );
    }
}

impl UiElement for TextInput {
    fn render(&mut self, ui: &mut Ui, _lua: Option<&Lua>) {
        ui.text_edit_singleline(&mut self.value);
    }
}

impl UiElement for ComboBoxInput {
    fn render(&mut self, ui: &mut Ui, _lua: Option<&Lua>) {
        egui::ComboBox::new(
            format!("{}{}{}", &self.name, &self.label, &self.id),
            &self.label,
        )
        .selected_text(format!("{:?}", &mut self.value))
        .width(170.0)
        .show_ui(ui, |ui| {
            for entry in &self.entries {
                ui.selectable_value(&mut self.value, entry.to_string(), format!("{:?}", entry));
            }
        });
    }
}

impl UiElement for ButtonInput {
    fn render(&mut self, ui: &mut Ui, lua: Option<&Lua>) {
        if ui.button(&self.name).clicked() {
            let callback: Function<'_> = lua
                .unwrap()
                .globals()
                .get(self.callback.to_owned())
                .expect("Button callback not found");
            callback.call::<_, ()>(()).unwrap();
        }
    }
}

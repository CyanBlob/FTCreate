use std::ops::RangeInclusive;
use egui::Ui;
use mlua::{MetaMethod, UserData, UserDataFields, UserDataMethods};
use crate::app::generators::control::Control;
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

impl UserData for Slider {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |_, this| Ok(this.value));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("set_value", |_, this, value: f32| {
            this.value = value;
            Ok(())
        });

        methods.add_method_mut("set_min", |_, this, value: f32| {
            this.min = value;
            Ok(())
        });

        methods.add_method_mut("set_max", |_, this, value: f32| {
            this.max = value;
            Ok(())
        });

        methods.add_method_mut("set_label", |_, this, value: String| {
            this.label = value.to_string();
            Ok(())
        });
    }
}

impl Control for Slider {
    fn render(&mut self, ui: &mut Ui) {
        ui.add(
            egui::Slider::new(&mut self.value, RangeInclusive::new(self.min, self.max))
                .text(&self.label)
                .max_decimals(self.deicimals),
        );
    }
}
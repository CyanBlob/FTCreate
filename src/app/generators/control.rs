use crate::app::generators::ui_elements::{
    ButtonInput, CheckboxInput, ComboBoxInput, Slider, TextInput,
};
use egui::Ui;
use mlua::prelude::LuaUserData;
use mlua::{Lua, UserData, UserDataFields};

pub trait SizedUserData: LuaUserData + Sized {}

pub trait UiElement {
    fn render(&mut self, ui: &mut Ui, lua: Option<&Lua>);
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[allow(unused)]
pub enum Control {
    SliderType(Slider),
    TextInputType(TextInput),
    ComboBoxType(ComboBoxInput),
    Label(String),
    CheckboxType(CheckboxInput),
    ButtonType(ButtonInput),
    Separator,
    Spacer,
}

impl UserData for Control {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |_, this| {
            return match this {
                Control::SliderType(s) => Ok(s.value),
                Control::CheckboxType(b) => match b.value {
                    true => Ok(1.0),
                    false => Ok(0.0),
                },
                _ => Ok(0.0),
            };
        });

        fields.add_field_method_set("value", |_, this, val: f32| {
            match this {
                Control::SliderType(s) => {
                    s.value = val;
                }
                Control::CheckboxType(c) => match val {
                    v if v > 0.0 => c.value = true,
                    v if v == 0.0 => c.value = false,
                    _ => {}
                },
                _ => {}
            }
            Ok(())
        });

        fields.add_field_method_get("text", |_, this| {
            return match this {
                Control::Label(l) => Ok(l.to_string()),
                Control::TextInputType(t) => Ok(t.value.to_string()),
                Control::ComboBoxType(c) => Ok(c.value.to_string()),
                Control::SliderType(s) => Ok(s.value.to_string()),
                _ => Ok("".to_string()),
            };
        });

        fields.add_field_method_set("text", |_, this, val: String| {
            match this {
                Control::ComboBoxType(c) => {
                    c.value = val;
                }
                Control::TextInputType(t) => {
                    t.value = val;
                }
                _ => {}
            }
            Ok(())
        });
    }
}

impl Control {
    pub fn render(&mut self, ui: &mut Ui, lua: &Lua) {
        match self {
            Control::SliderType(s) => {
                s.render(ui, None);
            }
            Control::TextInputType(t) => {
                t.render(ui, None);
            }
            Control::ComboBoxType(c) => c.render(ui, None),
            Control::ButtonType(b) => b.render(ui, Some(lua)),
            Control::CheckboxType(b) => {
                ui.checkbox(&mut b.value, &b.label);
            }
            Control::Separator => {
                ui.separator();
            }
            Control::Spacer => {
                ui.add_space(10.0);
            }
            Control::Label(l) => {
                ui.separator();
                ui.label(l.to_string());
            }
        }
    }

    pub fn get_name(&self) -> String {
        return match self {
            Control::SliderType(s) => s.name.to_string(),
            Control::Label(l) => l.to_string(),
            Control::TextInputType(t) => t.name.to_string(),
            Control::ComboBoxType(c) => c.name.to_string(),
            Control::CheckboxType(c) => c.name.to_string(),
            Control::ButtonType(b) => b.name.to_string(),
            _ => "".to_string(),
        };
    }
}

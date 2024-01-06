use egui::Ui;
use mlua::prelude::LuaUserData;
use mlua::{UserData, UserDataFields};
use crate::app::generators::ui_elements::{CheckboxInput, ComboBoxInput, Slider, TextInput};

pub trait SizedUserData: LuaUserData + Sized {}

pub trait UiElement {
    fn render(&mut self, ui: &mut Ui);
}

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(unused)]
pub enum Control
{
    SliderType(Slider),
    TextInputType(TextInput),
    ComboBoxType(ComboBoxInput),
    Label(String),
    CheckboxType(CheckboxInput),
    Separator,
    Spacer,
}

impl UserData for Control {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |_, this| {
            return match this {
                Control::SliderType(s) => {
                    Ok(s.value)
                }
                Control::CheckboxType(b) => {
                    match b.value {
                        true => { Ok(1.0) }
                        false => { Ok(0.0) }
                    }
                }
                _ => {
                    Ok(0.0)
                }
            };
        });

        fields.add_field_method_set("value", |_, this, val: f32| {
            match this {
                Control::SliderType(s) => {
                    s.value = val;
                },
                Control::CheckboxType(c) => {
                    match val {
                        v if v > 0.0 => { c.value = true}
                        v if v == 0.0 => { c.value = false}
                        _ => {}
                    }
                }
                _ => {}
            }
            Ok(())
        });

        fields.add_field_method_get("text", |_, this| {
            return match this {
                Control::Label(l) => {
                    Ok(l.to_string())
                }
                Control::TextInputType(t) => {
                    Ok(t.value.to_string())
                }
                Control::ComboBoxType(c) => {
                    Ok(c.value.to_string())
                }
                Control::SliderType(s) => {
                    Ok(s.value.to_string())
                }
                _ => {
                    Ok("".to_string())
                }
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

impl Control
{
    pub fn render(&mut self, ui: &mut Ui)
    {
        match self {
            Control::SliderType(s) => { s.render(ui); }
            Control::TextInputType(t) => { t.render(ui); }
            Control::ComboBoxType(c) => { c.render(ui) }
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

    pub fn get_name(&self) -> String
    {
        return match self {
            Control::SliderType(s) => { s.name.to_string() }
            Control::Label(l) => { l.to_string() }
            Control::TextInputType(t) => { t.name.to_string() }
            Control::ComboBoxType(c) => { c.name.to_string() }
            Control::CheckboxType(c) => { c.name.to_string() }
            _ => { "".to_string() }
        };
    }
}
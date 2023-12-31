use egui::Ui;
use mlua::prelude::LuaUserData;
use mlua::{UserData, UserDataFields};
use crate::app::generators::ui_elements::{ComboBoxInput, Slider, TextInput};

pub trait SizedUserData: LuaUserData + Sized {}

pub trait UiElement {
    fn render(&mut self, ui: &mut Ui);
}

#[derive(Clone, Debug)]
#[allow(unused)]
pub enum Control
{
    SliderType(Slider),
    TextInputType(TextInput),
    ComboBoxType(ComboBoxInput),
    Label(String),
}

impl UserData for Control {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |_, this| {
            return match this {
                Control::SliderType(s) => {
                    Ok(s.value)
                }
                Control::Label(l) => {
                    Ok(0.0)
                }
                Control::TextInputType(t) => {
                    Ok(0.0)
                }
                Control::ComboBoxType(c) => {
                    Ok(0.0)
                }
            };
        });

        fields.add_field_method_get("text", |_, this| {
            return match this {
                Control::SliderType(s) => {
                    Ok("".to_string())
                }
                Control::Label(l) => {
                    Ok(l.to_string())
                }
                Control::TextInputType(t) => {
                    Ok(t.value.to_string())
                }
                Control::ComboBoxType(c) => {
                    Ok(c.value.to_string())
                }
            };
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
            Control::ComboBoxType(c) => {c.render(ui)}
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
        };
    }
}
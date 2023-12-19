use egui::Ui;
use mlua::prelude::LuaUserData;
use mlua::{UserData, UserDataFields};
use crate::app::generators::slider::Slider;

pub trait SizedUserData: LuaUserData + Sized {}

pub trait UiElement {
    fn render(&mut self, ui: &mut Ui);
}

#[derive(Clone, Debug)]
#[allow(unused)]
pub enum Control
{
    SliderType(Slider),
    B,
}

impl UserData for Control {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |_, this| {
            return match this {
                Control::SliderType(s) => {
                    Ok(s.value)
                }
                Control::B => {
                    Ok(0.0)
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
            Control::B => { println!("Render not implemented for this variant") }
        }
    }
}
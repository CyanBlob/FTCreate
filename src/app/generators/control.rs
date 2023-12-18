use egui::Ui;
use mlua::prelude::LuaUserData;
use mlua::{UserData, UserDataFields, UserDataMethods};
use crate::app::generators::slider::Slider;

pub trait SizedUserData: LuaUserData + Sized {}

pub trait Control {
    fn render(&mut self, ui: &mut Ui);

    //fn obj(&mut self) -> impl UserData;
}

#[derive(Clone)]
pub enum ControlTest
{
    Slider(Slider),
    B
}

impl UserData for ControlTest {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |_, this|

            match this {
                ControlTest::Slider(s) => {
                    return Ok(s.value);
                }
                ControlTest::B => {
                    return Ok(0.0);
                }
            });
    }
}

impl ControlTest
{
    pub fn render(&mut self, ui: &mut Ui)
    {
        match self {
            ControlTest::Slider(s) => {s.render(ui);}
            ControlTest::B => {println!("Render not implemented for this variant")}
        }
    }
}
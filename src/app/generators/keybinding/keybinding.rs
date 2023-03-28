use egui_extras::RetainedImage;
use serde::{Deserialize, Serialize};

use super::super::generator;

pub(crate) const GAMEPAD_IMAGE: &[u8] = include_bytes!("../../../../resources/gamepad_white.png");

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Keybinding {
    pub value: i32,
    pub button: Option<BooleanButton>,
}

pub struct AxisKeybinding {
    pub value: f32,
    pub axis: Option<Axis>,
}

impl Keybinding {
    pub fn new(value: i32) -> Self {
        Keybinding {
            value: value,
            button: None,
        }
    }
}

impl AxisKeybinding {
    pub fn new(value: f32) -> Self {
        AxisKeybinding {
            value: value,
            axis: None,
        }
    }
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Copy)]
pub enum BooleanButton {
    A,
    B,
    X,
    Y,
    LeftStick,
    RightStick,
    LeftBumper,
    RightBumper,
    Start,
    Back,
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Copy)]
pub enum Axis {
    LeftTrigger,
    RightTrigger,
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
}

impl generator::Generator for Keybinding {
    fn get_methods(&self) -> Vec<crate::app::generators::method::Method> {
        vec![]
    }

    fn render_options(&mut self, _ui: &mut egui::Ui, _id: usize) {}
}

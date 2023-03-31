use egui_extras::RetainedImage;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use super::super::generator;

pub(crate) const GAMEPAD_IMAGE: &[u8] = include_bytes!("../../../../resources/gamepad_white.png");

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Keybinding<T> {
    pub value: T,
    pub button: Option<BooleanButton>,
}

#[allow(unused)]
pub struct AxisKeybinding {
    pub value: f32,
    pub axis: Option<Axis>,
}

impl<T> Keybinding<T> {
    pub fn new(value: T) -> Self {
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Copy, EnumIter)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Copy, EnumIter)]
pub enum Axis {
    LeftTrigger,
    RightTrigger,
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
}

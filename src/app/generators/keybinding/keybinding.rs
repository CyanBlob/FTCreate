use serde::{Deserialize, Serialize};
use strum::EnumIter;

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
    #[allow(unused)]
    pub fn new(value: f32) -> Self {
        AxisKeybinding {
            value: value,
            axis: None,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Copy, EnumIter)]
pub enum BooleanButton {
    a,
    b,
    x,
    y,
    left_stick_button,
    right_stick_button,
    left_bumper,
    right_bumper,
    dpad_left,
    dpad_right,
    dpad_up,
    dpad_down,
    start,
    select,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Copy, EnumIter)]
pub enum Axis {
    left_trigger,
    right_trigger,
    left_stick_x,
    left_stick_y,
    right_stick_x,
    right_stick_y,
}

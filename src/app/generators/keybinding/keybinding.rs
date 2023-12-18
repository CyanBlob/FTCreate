use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Keybinding<T> {
    pub value: T,
    pub button: Option<BooleanButton>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct AxisKeybinding {
    pub reversed: bool,
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

impl Eq for Keybinding<i32> {}

impl Ord for Keybinding<i32> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.button {
            None => Ordering::Less,
            Some(button) => {
                match other.button {
                    None => Ordering::Greater,
                    Some(other) =>button.partial_cmp(&other).unwrap(),
                }
            }
        }
    }
}

impl Ord for Keybinding<f32> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.button {
            None => Ordering::Less,
            Some(button) => {
               match other.button {
                   None => Ordering::Greater,
                   Some(other) =>button.partial_cmp(&other).unwrap(),
               }
            }
        }
    }
}

impl Eq for Keybinding<f32> {}


impl AxisKeybinding {
    #[allow(unused)]
    pub fn new(value: f32) -> Self {
        AxisKeybinding {
            reversed: false,
            axis: None,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Copy, EnumIter)]
pub enum BooleanButton {
    default = 0,
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
    default,
    left_trigger,
    right_trigger,
    left_stick_x,
    left_stick_y,
    right_stick_x,
    right_stick_y,
}

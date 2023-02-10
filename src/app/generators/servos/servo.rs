use serde::{Serialize, Deserialize};

use strum_macros::EnumIter;

use crate::app::generators::generator::Generator;

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
pub enum ServoDirection {
    FORWARD,
    REVERSE
}

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
pub enum ServoMode {
    Servo,
    Continuous,
}

pub trait ServoGenerator: Servo + Generator {
    fn new(id: i32) -> Self;
}
pub trait Servo {}

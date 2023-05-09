use serde::{Deserialize, Serialize};

use strum_macros::EnumIter;

use crate::app::generators::generator::Generator;

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
pub enum ServoDirection {
    FORWARD,
    REVERSE,
}

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
pub enum ServoMode {
    Servo,
    Continuous,
}

pub trait ServoGenerator: Servo + Generator {
    fn new(name: String) -> Self where Self: Sized;
}
pub trait Servo {}

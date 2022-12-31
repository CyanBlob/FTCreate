use serde::{Serialize, Deserialize};

use strum_macros::EnumIter;

use crate::app::generators::generator::Generator;

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
pub enum MotorDirection {
    Forward,
    Reverse
}

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
pub enum MotorMode {
    RunToPosition,
    RunWithEncoders,
    RunWithoutEncoders
}

pub trait MotorGenerator: Motor + Generator {}
pub trait Motor {}

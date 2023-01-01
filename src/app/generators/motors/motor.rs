use serde::{Serialize, Deserialize};

use strum_macros::EnumIter;

use crate::app::generators::generator::Generator;

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
pub enum MotorDirection {
    FORWARD,
    REVERSE
}

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
#[allow(non_camel_case_types)]
pub enum MotorMode {
    RUN_TO_POSITION,
    RUN_WITH_ENCODERS,
    RUN_WITHOUT_ENCODERS
}

pub trait MotorGenerator: Motor + Generator {
    fn new() -> Self;
}
pub trait Motor {}

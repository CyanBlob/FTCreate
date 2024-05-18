use serde::{Deserialize, Serialize};

use strum_macros::EnumIter;

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
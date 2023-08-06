use serde::{Deserialize, Serialize};

use strum_macros::EnumIter;

use crate::app::generators::{generator::Generator, subsystem::subsystem::DrivetrainType};

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
pub enum MotorDirection {
    FORWARD,
    REVERSE,
}

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
#[allow(non_camel_case_types)]
pub enum MotorMode {
    RUN_TO_POSITION,
    RUN_USING_ENCODERS,
    RUN_WITHOUT_ENCODERS,
}

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
pub enum MecanumPosition {
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight,
}

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
pub enum ArcadePosition {
    Left,
    Right,
}

pub trait MotorGenerator: Motor + Generator {
    fn new(name: String) -> Self
    where
        Self: Sized;
    fn set_drivetrain_type(&mut self, drivetrain_type: Option<DrivetrainType>);
    fn set_mecanum_position(&mut self, position: MecanumPosition);
}
pub trait Motor {}

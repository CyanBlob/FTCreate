use serde::{Serialize, Deserialize};

use strum_macros::EnumIter;

use crate::app::generators::{generator::Generator, drivetrain::drivetrain::DrivetrainType};

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

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
pub enum MecanumPosition {
    FrontLeft,
    FrontRight,
    RearLeft,
    RearRight
}

#[derive(Copy, PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord, EnumIter)]
pub enum TankPosition {
    Left,
    Right,
}

pub trait MotorGenerator: Motor + Generator {
    fn new() -> Self;
    fn set_drivetrain_type(&mut self, drivetrain_type: Option<DrivetrainType>);
    fn set_mecanum_position(&mut self, position: MecanumPosition);
}
pub trait Motor {}

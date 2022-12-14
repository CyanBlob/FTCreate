//use crate::generators::drivetrain;
//use crate::generators::motors;
use crate::generators::motors::motor::Motor;
use super::super::generator;

#[cfg(allow_unused)]
enum DrivetrainType {
    MECANUM,
    TANK,
    SWERVE,
    ARCADE
}

pub struct Drivetrain {
    pub motors: Vec::<Box<dyn generator::Generator>>,
}
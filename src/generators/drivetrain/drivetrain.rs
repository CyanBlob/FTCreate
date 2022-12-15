//use crate::generators::drivetrain;
//use crate::generators::motors;
use crate::generators::motors::motor::MotorGenerator;

#[cfg(allow_unused)]
enum DrivetrainType {
    MECANUM,
    TANK,
    SWERVE,
    ARCADE
}

pub struct Drivetrain <T: MotorGenerator> {
    pub motors: Vec::<Box::<T>>,
}
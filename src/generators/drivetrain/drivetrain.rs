use crate::generators::motors::motor::MotorGenerator;
use druid::Data;
use druid::im::Vector;

#[cfg(allow_unused)]
enum DrivetrainType {
    MECANUM,
    TANK,
    SWERVE,
    ARCADE,
}

// TODO: vecs should be wrapped in Rc (or use Im vecs?)
#[derive(Debug, Clone, Data, PartialEq)]
pub struct Drivetrain<T: MotorGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone> {
    #[data(same_fn = "PartialEq::eq")]
    pub motors: Vector<Box<T>>,
}

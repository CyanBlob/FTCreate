use serde::{Serialize, Deserialize};

use crate::generators::generator::Generator;

#[derive(Serialize, Deserialize)]
pub enum MotorDirection {
    FORWARD,
    REVERSE
}

pub trait Motor {}

/*impl<T> Generator for T
where
    T: Motor,
{
    fn generate(&self) {
    }
}*/
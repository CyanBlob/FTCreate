use serde::{Serialize, Deserialize};

use crate::generators::generator::Generator;

#[derive(Serialize, Deserialize)]
pub enum MotorDirection {
    FORWARD,
    REVERSE
}

pub trait MotorGenerator: Motor + Generator {}
pub trait Motor {}
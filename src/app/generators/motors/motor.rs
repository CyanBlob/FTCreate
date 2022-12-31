use serde::{Serialize, Deserialize};

use crate::app::generators::generator::Generator;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, PartialOrd, Ord)]
pub enum MotorDirection {
    FORWARD,
    REVERSE
}

pub trait MotorGenerator: Motor + Generator {}
pub trait Motor {}

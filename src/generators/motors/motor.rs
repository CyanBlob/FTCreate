use serde::{Serialize, Deserialize};

use crate::generators::generator::Generator;
use druid::Data;

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone, Data, PartialOrd, Ord)]
pub enum MotorDirection {
    FORWARD,
    REVERSE
}

pub trait MotorGenerator: Motor + Generator {}
pub trait Motor {}

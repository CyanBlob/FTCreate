use serde::{Deserialize, Serialize};
use serde_json::Result;

use super::motor;
use super::super::generator;

use crate::generators::motors;

use druid::Data;

use motor::*;

#[derive(Serialize, Deserialize, Debug, Clone, Data, PartialEq, PartialOrd)]
pub struct CoreHD {
    pub direction: motors::motor::MotorDirection,
    pub max_speed: f32,
    pub position: u8
}

impl generator::Generator for CoreHD {
    fn generate(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
    }

    fn serialize(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
    }

    fn deserialize(&self, json: &str) -> Result<Box::<Self>> {
        match serde_json::from_str::<Self>(json) {
            Ok(s) => {
                Ok(Box::new(s))
            }
            Err(e) => {
                Err(e)
            }
            
        }
    }
}

impl Motor for CoreHD {}

impl MotorGenerator for CoreHD {}

use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use super::motor;
use super::super::generator;

use crate::generators::motors;

use motor::*;

#[derive(Serialize, Deserialize)]
pub struct CoreHD {
    pub direction: motors::motor::MotorDirection,
    pub max_speed: f32,
    pub position: u8
}

impl Motor for CoreHD {}

impl generator::Generator for CoreHD {
    fn generate(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
    }
}
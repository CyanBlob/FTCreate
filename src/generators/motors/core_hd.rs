use serde::{Deserialize, Serialize};
use serde_json::Result;

use super::motor;
use super::super::generator;

use crate::generators::motors;

use druid::{Data, Lens, widget::{Padding, Flex, Slider, Label, TextBox}, WidgetExt,  text::ParseFormatter};

use motor::*;

#[derive(Serialize, Deserialize, Debug, Clone, Data, PartialEq, PartialOrd, Lens)]
pub struct CoreHD {
    pub direction: motors::motor::MotorDirection,
    pub max_speed: f64,
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

    fn render_options(&self) -> Box::<dyn druid::Widget<Self>> {
        Box::new(Padding::new(0.0,
            Flex::column()
                .with_flex_child(Label::new("Core HD Motor"), 1.0)
                .with_flex_child(TextBox::new().with_formatter(ParseFormatter::new()).update_data_while_editing(true).lens(CoreHD::max_speed).fix_width(100.0), 1.0)
                .with_flex_child(Slider::new().with_range(-1.0, 1.0).lens(CoreHD::max_speed).padding(10.0), 1.0)
        ))
    }
}

impl Motor for CoreHD {}

impl MotorGenerator for CoreHD {}

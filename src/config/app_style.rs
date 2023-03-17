use eframe::egui::Color32;
use eframe::egui::Rounding;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStyle {
    pub background: Color32,
    pub foreground: Color32,

    pub circle_focus: Color32,
    pub circle_short_break: Color32,
    pub circle_long_break: Color32,

    pub robochargers_blue: Color32,
    pub robochargers_yellow: Color32,

    pub bg: Color32,
    pub bg_light: Color32,

    pub accent_blue: Color32,
    pub accent_yellow: Color32,

    pub text_color: Color32,
    pub input_text_color: Color32,

    pub dropdown_rounding: Rounding,
}

impl Default for AppStyle {
    fn default() -> Self {
        Self {
            background: Color32::from_rgb(0, 255, 255),
            foreground: Color32::from_rgb(255, 0, 0),

            circle_focus: Color32::from_rgb(255, 255, 0),
            circle_short_break: Color32::from_rgb(0, 255, 0),
            circle_long_break: Color32::from_rgb(0, 0, 255),

            robochargers_blue: Color32::from_rgb(1, 71, 250),
            robochargers_yellow: Color32::from_rgb(255, 236, 0),

            bg: Color32::from_rgb(0, 18, 62),
            bg_light: Color32::from_rgb(64, 59, 0),

            accent_blue: Color32::from_rgb(1, 71, 250),
            accent_yellow: Color32::from_rgb(255, 236, 0),

            text_color: Color32::from_rgb(255, 255, 255),
            input_text_color: Color32::from_rgb(200, 200, 200),

            dropdown_rounding: 3.0.into(),
        }
    }
}

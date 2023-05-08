use crate::config::AppStyle;
use eframe::egui::{Visuals};

pub struct Theme {
    pub visuals: Visuals,
}

impl Theme {
    pub fn new(cfg: &AppStyle) -> Self {
        let mut visuals = Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = cfg.background;
        visuals.widgets.noninteractive.fg_stroke.color = cfg.text_color;
        visuals.widgets.noninteractive.rounding = cfg.dropdown_rounding;

        // colors for things that are currently being clicked
        visuals.widgets.active.bg_fill = cfg.accent_blue;
        visuals.widgets.active.fg_stroke.color = cfg.accent_yellow;
        visuals.widgets.active.rounding = cfg.dropdown_rounding;

        // hovered elements
        visuals.widgets.hovered.bg_fill = cfg.accent_blue;

        // used in text edits
        visuals.widgets.inactive.fg_stroke.color = cfg.input_text_color;
        visuals.widgets.inactive.rounding = cfg.dropdown_rounding;

        visuals.widgets.inactive.bg_fill = cfg.accent_blue;

        // dropdown "buttons"
        visuals.widgets.inactive.bg_fill = cfg.bg_light;

        // dropdown backdrops
        visuals.window_fill = cfg.bg_light;

        // background
        visuals.panel_fill = cfg.bg;

        Self {
            visuals,
        }
    }
}

use crate::config::AppStyle;
use eframe::egui::{Style, Visuals};

pub struct Theme {
    pub visuals: Visuals,
    pub slider: Style,
    pub checkbox: Style,
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
        

        let mut slider = Style {
            visuals: visuals.clone(),
            ..Default::default()
        };
        slider.visuals.widgets.inactive.bg_fill = cfg.bg;
        slider.visuals.widgets.inactive.fg_stroke.width = 0.0;
        slider.visuals.widgets.inactive.expansion = 3.0;

        slider.visuals.widgets.hovered = slider.visuals.widgets.inactive;
        slider.visuals.widgets.active = slider.visuals.widgets.inactive;

        let mut checkbox = Style {
            visuals: visuals.clone(),
            ..Default::default()
        };
        checkbox.visuals.widgets.noninteractive.fg_stroke.color = cfg.foreground;
        checkbox.visuals.widgets.noninteractive.bg_stroke.width = 0.5;
        checkbox.visuals.widgets.noninteractive.bg_stroke.color = cfg.foreground;
        checkbox.visuals.widgets.noninteractive.expansion = 1.0;
        checkbox.spacing.icon_spacing = 10.0;

        checkbox.visuals.widgets.inactive = checkbox.visuals.widgets.noninteractive;
        checkbox.visuals.widgets.active = checkbox.visuals.widgets.noninteractive;
        checkbox.visuals.widgets.hovered = checkbox.visuals.widgets.noninteractive;

        Self {
            visuals,
            slider,
            checkbox,
        }
    }

    pub fn visuals(&self) -> &Visuals {
        &self.visuals
    }

    pub fn slider(&self) -> &Style {
        &self.slider
    }

    pub fn checkbox(&self) -> &Style {
        &self.checkbox
    }
}
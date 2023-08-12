use crate::config::AppStyle;
use eframe::egui::Visuals;

pub struct Theme {
    pub visuals: Visuals,
}

impl Theme {
    pub fn new(_cfg: &AppStyle) -> Self {
        let visuals = Visuals::dark();

        //visuals.widgets.noninteractive.bg_fill = cfg.background; visuals.widgets.noninteractive.fg_stroke.color = cfg.text_color;
        //visuals.widgets.noninteractive.rounding = cfg.dropdown_rounding;

        // colors for things that are currently being clicked
        //visuals.widgets.active.bg_fill = cfg.accent_blue;
        //visuals.widgets.active.fg_stroke.color = cfg.accent_yellow;
        //visuals.widgets.active.rounding = cfg.dropdown_rounding;

        // hovered elements
        //visuals.widgets.hovered.weak_bg_fill = cfg.accent_blue;
        //visuals.widgets.hovered.bg_fill = cfg.accent_green;

        // used in text edits
        //visuals.widgets.inactive.fg_stroke.color = cfg.input_text_color;
        //visuals.widgets.inactive.rounding = cfg.dropdown_rounding;

        //visuals.widgets.inactive.bg_fill = cfg.accent_blue;

        // combobox "buttons"
        //visuals.widgets.inactive.bg_fill = cfg.bg_light;
        //visuals.widgets.inactive.weak_bg_fill = cfg.accent_blue;
        //visuals.widgets.open.weak_bg_fill = cfg.accent_blue;
        //visuals.widgets.open.bg_fill = cfg.accent_blue;

        // selected combobox entry while expanded
        //visuals.selection.bg_fill = cfg.accent_green;

        // combobox "click" color
        //visuals.widgets.active.weak_bg_fill = cfg.accent_yellow;

        // slider fill color (when clicked)
        //visuals.widgets.active.bg_fill = cfg.accent_green;

        // dropdown backdrops
        //visuals.window_fill = cfg.bg_light;

        // background
        //visuals.panel_fill = cfg.bg;

        Self { visuals }
    }
}

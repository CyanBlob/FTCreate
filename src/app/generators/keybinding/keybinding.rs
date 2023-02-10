use super::super::generator;

struct Keybinding {}

pub enum BooleanButton {
    A,
    B,
    X,
    Y,
    LeftStick,
    RightStick,
    LeftBumper,
    RightBumper,
    Start,
    Back,
}

pub enum Axis {
    LeftTrigger,
    RightTrigger,
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
}

impl generator::Generator for Keybinding {
    fn get_methods(&self) -> Vec<crate::app::generators::method::Method> {
        vec![]
    }

    fn render_options(&mut self, ui: &mut egui::Ui, id: usize) {}
}

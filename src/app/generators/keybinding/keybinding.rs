use super::super::generator;

struct Keybinding {}

#[allow(unused)]
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

#[allow(unused)]
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

    fn render_options(&mut self, _ui: &mut egui::Ui, _id: usize) {}
}

use eframe::egui;
use egui_dnd::dnd;
use ordered_float::OrderedFloat;
use strum_macros::EnumIter;
use crate::app::auton::auton::AutonCommand::{DriveBackward, DriveForward, StrafeLeft, StrafeRight, RotateLeft, RotateRight, Wait}; // standard floats don't implement Hash, which is required for egui_dnd items

pub struct Auton {
    commands: Vec<AutonCommand>,
}

#[derive(Hash, Debug, EnumIter)]
pub enum AutonCommand {
    DriveForward { speed: OrderedFloat<f32>, distance: OrderedFloat<f32> },
    DriveBackward { speed: OrderedFloat<f32>, distance: OrderedFloat<f32> },
    StrafeLeft { speed: OrderedFloat<f32>, distance: OrderedFloat<f32> },
    StrafeRight { speed: OrderedFloat<f32>, distance: OrderedFloat<f32> },
    RotateLeft { speed: OrderedFloat<f32>, distance: OrderedFloat<f32> },
    RotateRight { speed: OrderedFloat<f32>, distance: OrderedFloat<f32> },
    Wait { time: OrderedFloat<f32> },
}

impl AutonCommand {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        match self {
            DriveForward { speed, distance } => { Self::render_drive("Drive Forward", speed, distance, ui); }
            DriveBackward { speed, distance } => { Self::render_drive("Drive Backward", speed, distance, ui); }
            StrafeLeft { speed, distance } => { Self::render_drive("Strafe Left", speed, distance, ui); }
            StrafeRight { speed, distance } => { Self::render_drive("Strafe Right", speed, distance, ui); }
            RotateLeft { speed, distance } => { Self::render_drive("Rotate Left", speed, distance, ui); }
            RotateRight { speed, distance } => { Self::render_drive("Rotate Right", speed, distance, ui); }
            Wait { time } => {
                ui.label(format!("Wait {:?} seconds", time.0));
            }
        }
    }

    pub fn render_drive(name: &str, speed: &mut OrderedFloat<f32>, distance: &mut OrderedFloat<f32>, ui: &mut egui::Ui) {
        ui.label(name);
        ui.add(
            egui::Slider::new(&mut speed.0, -1.0..=1.0)
                .text("Speed")
                .step_by(0.01)
                .max_decimals(2),
        );

        ui.horizontal(|ui| {
            ui.add(
                egui::DragValue::new(&mut distance.0).max_decimals(2).speed(0.1)
            );
            ui.label("Distance");
        });
    }
}

impl Auton {
    pub fn new() -> Self {
        let commands = vec![
            DriveForward { speed: OrderedFloat::from(1.0), distance: OrderedFloat::from(2.0) },
            DriveBackward { speed: OrderedFloat::from(5.0), distance: OrderedFloat::from(10.0) },
            StrafeLeft { speed: OrderedFloat::from(3.0), distance: OrderedFloat::from(2.5) },
            DriveForward { speed: OrderedFloat::from(-1.0), distance: OrderedFloat::from(2.0) },
            DriveBackward { speed: OrderedFloat::from(-5.0), distance: OrderedFloat::from(10.0) },
            StrafeLeft { speed: OrderedFloat::from(-3.0), distance: OrderedFloat::from(2.5) },
            Wait { time: OrderedFloat::from(2.5) },
        ];

        Auton {
            commands,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        dnd(ui, "auton_commands").show_vec(&mut self.commands, |ui, item, handle, state| {
            ui.horizontal(|ui| {
                handle.ui(ui, |ui| {
                    if state.dragged {
                        ui.label("dragging");
                    } else {
                        ui.label("drag");
                    }
                });
                item.render(ui);
            });
        });
    }
}
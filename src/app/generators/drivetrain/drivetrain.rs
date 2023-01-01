use crate::app::generators::{generator, motors::motor::MotorGenerator};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, EnumIter, PartialOrd, Copy,
)]
pub enum DrivetrainType {
    Mecanum,
    Tank,
    //Swerve,
    //Arcade,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Drivetrain<
    T: MotorGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
> {
    pub motors: Vec<T>,
    pub drivetrain_type: DrivetrainType,
}

impl<T: MotorGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone>
    generator::Generator for Drivetrain<T>
{
    fn render_options(&mut self, ui: &mut egui::Ui, _id: usize) {
        //ui.label("Drivetrain options");
        ui.add_space(10.0);

        if ui.button("Add motor").clicked() {
            self.motors.push(T::new());
        }

        if ui.button("Remove motor").clicked() {
            self.motors.pop();
        }

        ui.add_space(20.0);

        egui::ComboBox::from_label("Drivetrain type")
            .selected_text(format!("{:?}", &mut self.drivetrain_type))
            .width(170.0)
            .show_ui(ui, |ui| {
                for mode in DrivetrainType::iter() {
                    ui.selectable_value(&mut self.drivetrain_type, mode, format!("{:?}", mode));
                }
            });

        self.motors.iter_mut().for_each(|motor| {
            motor.set_drivetrain_type(self.drivetrain_type);
        });

        self.motors.iter_mut().enumerate().for_each(|(id, motor)| {
            ui.add_space(20.0);
            ui.separator();
            motor.render_options(ui, id);
        });
    }

    fn generate_loop_one_time_setup(&self) -> String {
        format!("\t\t\t// Drivetrain one time setup\n\t\t\tdouble drive  = -gamepad1.left_stick_y*driveSpeed;  // forwards and backwards movement\n\
        \t\t\tdouble strafe = -gamepad1.left_stick_x*driveSpeed;  // side to side movement\n\
        \t\t\tdouble turn   =  gamepad1.right_stick_x*turnSpeed;  // rotation\n\n")
    }
}

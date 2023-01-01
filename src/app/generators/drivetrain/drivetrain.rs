use crate::app::generators::{motors::{motor::MotorGenerator}, generator};

#[cfg(allow_unused)]
enum DrivetrainType {
    MECANUM,
    TANK,
    SWERVE,
    ARCADE,
}

// TODO: vecs should be wrapped in Rc (or use Im vecs?)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Drivetrain<T: MotorGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone> {
    pub motors: Vec<T>,
}

impl<T: MotorGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone> generator::Generator for Drivetrain<T> {
    fn render_options(&mut self, ui: &mut egui::Ui, _id: usize) {
        //ui.label("Drivetrain options");
        ui.add_space(10.0);
        
        if ui.button("Add motor").clicked() {
            self.motors.push(T::new());
        }

        if ui.button("Remove motor").clicked() {
            self.motors.pop();
        }
        
            self
                .motors
                .iter_mut()
                .enumerate()
                .for_each(|(id, motor)| {
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
use crate::app::generators::{
    generator::{self, SubsystemGenerator},
    motors::{dc_motor::DcMotor, motor::MotorGenerator},
    servos::servo::ServoGenerator,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Subsystem<
    T: MotorGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
    U: ServoGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
> {
    pub motors: Vec<T>,
    pub servos: Vec<U>,
    pub name: String,
}

impl<
        T: MotorGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
        U: ServoGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
    > generator::Generator for Subsystem<T, U>
{
    fn generate_includes(&self) -> String {
        if self.motors.len() > 0 as usize {
            return self.motors.iter().nth(0).unwrap().generate_includes();
        }
        if self.servos.len() > 0 as usize {
            return self.servos.iter().nth(0).unwrap().generate_includes();
        }
        "".to_string()
    }

    fn generate_globals(&self) -> String {
        let mut code = "".to_string();

        if self.motors.len() > 0 as usize {
            self.motors.iter().for_each(|motor| {
                code += &motor.generate_globals();
            });
        }

        if self.servos.len() > 0 as usize {
            self.servos.iter().for_each(|servos| {
                code += &servos.generate_globals();
            });
        }
        code
    }

    fn generate_init(&self) -> String {
        let mut code = "".to_string();

        if self.motors.len() > 0 as usize {
            self.motors.iter().for_each(|motor| {
                code += &motor.generate_init();
            });
        }
        if self.servos.len() > 0 as usize {
            self.servos.iter().for_each(|servos| {
                code += &servos.generate_init();
            });
        }
        code
    }

    fn generate_loop_one_time_setup(&self) -> String {
        let mut code = "".to_string();
        if self.motors.len() > 0 as usize {
            self.motors.iter().for_each(|motor| {
                code += &motor.generate_loop_one_time_setup();
            });
        }

        if self.servos.len() > 0 as usize {
            self.servos.iter().for_each(|servos| {
                code += &servos.generate_loop_one_time_setup();
            });
        }

        code
    }

    fn generate_loop(&self) -> String {
        let mut code = "".to_string();

        if self.motors.len() > 0 as usize {
            self.motors.iter().for_each(|motor| {
                code += &motor.generate_loop();
            });
        }

        if self.servos.len() > 0 as usize {
            self.servos.iter().for_each(|servos| {
                code += &servos.generate_loop();
            });
        }
        code
    }
    fn render_options(&mut self, ui: &mut egui::Ui, _id: usize) {
        ui.add_space(30.0);

        let mut added_motors = 0;
        let num_columns = 2;

        ui.horizontal(|ui| {
            ui.heading("Motors");

            if ui.button("Add motor").clicked() {
                self.motors.push(T::new());
            }

            if ui.button("Remove motor").clicked() {
                self.motors.pop();
            }
        });

        ui.add_space(10.0);

        egui::Grid::new("Subsystem motors grid").show(ui, |ui| {
            self.motors.iter_mut().enumerate().for_each(|(id, motor)| {
                added_motors += 1;
                ui.vertical(|ui| {
                    ui.add_space(20.0);
                    ui.separator();
                    motor.render_options(ui, id);
                });

                if added_motors % num_columns == 0 {
                    ui.end_row();
                }
            });
        });

        added_motors = 0;

        ui.horizontal(|ui| {
            ui.heading("Servos");

            if ui.button("Add servo").clicked() {
                self.servos.push(U::new());
            }

            if ui.button("Remove servo").clicked() {
                self.servos.pop();
            }
        });

        egui::Grid::new("Servos grid").show(ui, |ui| {
            self.servos.iter_mut().enumerate().for_each(|(id, servo)| {
                added_motors += 1;
                ui.vertical(|ui| {
                    ui.add_space(20.0);
                    ui.separator();
                    servo.render_options(ui, id + 1000);
                });

                if added_motors % num_columns == 0 {
                    ui.end_row();
                }
            });
        });
    }
}

impl<
        T: MotorGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
        U: ServoGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
    > SubsystemGenerator for Subsystem<T, U>
{
    /*fn new() -> Self {
        Subsystem {
            motors: vec![T::new(), T::new()],
            servos: vec![],
            name: "Subsystem".to_string(),
        }
    }*/

    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl<
        T: MotorGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
        U: ServoGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone,
    > Subsystem<T, U>
{
    pub fn new() -> Self {
        Subsystem {
            motors: vec![T::new(), T::new()],
            servos: vec![],
            name: "Subsystem".to_string(),
        }
    }
}

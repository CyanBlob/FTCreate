use crate::app::generators::{
    generator::{self, SubsystemGenerator},
    motors::motor::MotorGenerator,
    servos::servo::ServoGenerator,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::app::generators::lua_generator::{ControlHandler, LuaGenerator};

#[derive(
Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, EnumIter, PartialOrd, Copy,
)]
pub enum DrivetrainType {
    Mecanum,
    Arcade,
    Tank,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Subsystem<
    T: MotorGenerator + PartialEq + PartialOrd + Clone,
    U: ServoGenerator + PartialEq + PartialOrd + Clone,
> {
    pub motors: Vec<T>,
    pub servos: Vec<U>,
    pub is_drivetrain: bool,
    pub drivetrain_type: DrivetrainType,
    pub invert_steering: bool,
    pub name: String,
    pub control_handler: ControlHandler,
    pub selected_control: String,
}

impl<
    T: MotorGenerator + PartialEq + PartialOrd + Clone,
    U: ServoGenerator + PartialEq + PartialOrd + Clone,
> generator::Generator for Subsystem<T, U>
{
    fn generate_includes(&self) -> String {
        let mut code: String = "".to_owned();
        if self.motors.len() > 0usize {
            code += &self.motors.iter().nth(0).unwrap().generate_includes();
        }
        if self.servos.len() > 0usize {
            code += &self.servos.iter().nth(0).unwrap().generate_includes();
        }

        code += &*self.control_handler.generate_includes().to_string();

        code
    }

    fn generate_globals(&self) -> String {
        let mut code = "".to_string();

        if self.motors.len() > 0usize {
            self.motors.iter().for_each(|motor| {
                code += &motor.generate_globals();
            });
        }

        if self.servos.len() > 0usize {
            self.servos.iter().for_each(|servos| {
                code += &servos.generate_globals();
            });
        }

        code += &*self.control_handler.generate_globals().to_string();

        code
    }

    fn generate_init(&self) -> String {
        let mut code = "".to_string();

        if self.motors.len() > 0usize {
            self.motors.iter().for_each(|motor| {
                code += &motor.generate_init();
            });
        }

        if self.servos.len() > 0usize {
            self.servos.iter().for_each(|servos| {
                code += &servos.generate_init();
            });
        }

        code += &*self.control_handler.generate_init().to_string();

        code
    }

    fn generate_loop_one_time_setup(&self) -> String {
        let mut code: String = "".to_owned();
        if self.is_drivetrain {
            match self.drivetrain_type {
                DrivetrainType::Mecanum => {
                    code += &"\t\t\t// Mecanum drivetrain one time setup\n\t\t\tdouble drive  = gamepad1.left_stick_y;  // forwards and backwards movement\n\
            \t\t\tdouble turn   = gamepad1.right_stick_x; // rotation\n".to_string();

                    code += &"\t\t\tdouble strafe = gamepad1.left_stick_x;  // side to side movement\n".to_string();
                }
                DrivetrainType::Arcade => {
                    code += &"\t\t\t// Arcade drivetrain one time setup\n\t\t\tdouble drive  = gamepad1.left_stick_y;  // forwards and backwards movement\n\
                    \t\t\tdouble turn   = gamepad1.right_stick_x; // rotation\n".to_string();
                }
                DrivetrainType::Tank => {
                    code += &"\t\t\t// Arcade drivetrain one time setup\n\t\t\tdouble driveLeft  = gamepad1.left_stick_y;  // left motors movement\n\
                    \t\t\tdouble driveRight = gamepad1.right_stick_y; // right motors movement\n".to_string();
                }
            }

            match self.drivetrain_type {
                DrivetrainType::Tank => {}
                _ => match self.invert_steering {
                    true => code += "\n\t\t\tturn = turn * -1.0;\n",
                    false => {}
                },
            }

            code += "\n";
        }

        /*if self.motors.len() > 0 as usize {
            self.motors.iter().for_each(|motor| {
                code += &motor.generate_loop_one_time_setup();
            });
        }*/

        if self.servos.len() > 0 as usize {
            self.servos.iter().for_each(|servos| {
                code += &servos.generate_loop_one_time_setup();
            });
        }

        code += &*self.control_handler.generate_loop_one_time_setup().to_string();

        code
    }

    fn generate_loop(&self) -> String {
        let mut code = "".to_string();

        /*if self.motors.len() > 0usize {
            self.motors.iter().for_each(|motor| {
                code += &motor.generate_loop();
            });
        }*/

        if self.servos.len() > 0usize {
            self.servos.iter().for_each(|servos| {
                code += &servos.generate_loop();
            });
        }

        code += &*self.control_handler.generate_loop().to_string();

        code
    }

    fn render_options(&mut self, ui: &mut egui::Ui, _id: usize) {
        egui::scroll_area::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.add_space(20.0);

                egui::ComboBox::new(format!("{}.{}", self.name, "Controls"), "Controls List")
                    .selected_text(format!("{}", &mut self.selected_control))
                    .width(170.0)
                    .show_ui(ui, |ui| {
                        for script in &self.control_handler.scripts {
                            let script_name = script.split("/").last().unwrap().split(".").nth(0).unwrap();
                            ui.selectable_value(
                                &mut self.selected_control,
                                script.clone(),
                                format!("{:?}", script_name),
                            );
                        }
                    });

                if ui.button("Add selected control").clicked() {
                    let mut new_generator = LuaGenerator::new(&self.selected_control);
                    new_generator.load();
                    self.control_handler.generators.push(new_generator);
                }

                self.control_handler.render(ui);

                if self.is_drivetrain {
                    egui::ComboBox::from_label("Drivetrain type")
                        .selected_text(format!("{:?}", &mut self.drivetrain_type))
                        .width(170.0)
                        .show_ui(ui, |ui| {
                            for mode in DrivetrainType::iter() {
                                ui.selectable_value(
                                    &mut self.drivetrain_type,
                                    mode,
                                    format!("{:?}", mode),
                                );
                            }
                        });

                    match self.drivetrain_type {
                        DrivetrainType::Tank => {}
                        _ => {
                            egui::ComboBox::from_label("Invert steering")
                                .selected_text(format!("{}", self.invert_steering))
                                .width(170.0)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.invert_steering, false, "false");
                                    ui.selectable_value(&mut self.invert_steering, true, "true");
                                });

                            self.motors.iter_mut().for_each(|motor| {
                                motor.set_drivetrain_type(Some(self.drivetrain_type));
                            });
                        }
                    }
                }

                let mut num_motors = 0;
                let num_columns = 2;

                ui.add_space(30.0);

                ui.horizontal(|ui| {
                    //ui.heading("Motors");

                    /*if ui.button("Add motor").clicked() {
                        self.motors.push(T::new(format!(
                            "{}_motor_{}",
                            self.name,
                            self.motors.len() as i32 + 1
                        )));
                    }

                    if ui.button("Remove motor").clicked() {
                        self.motors.pop();
                    }*/
                });

                ui.add_space(10.0);

                egui::Grid::new("Drivetrain motors grid").show(ui, |ui| {
                    self.motors.iter_mut().enumerate().for_each(|(id, motor)| {
                        num_motors += 1;
                        ui.vertical(|ui| {
                            ui.add_space(20.0);
                            ui.separator();
                            motor.render_options(ui, id);
                        });

                        if num_motors % num_columns == 0 {
                            ui.end_row();
                        }
                    });
                });

                ui.add_space(30.0);

                ui.horizontal(|ui| {
                    ui.heading("Servos");

                    if ui.button("Add servo").clicked() {
                        self.servos.push(U::new(format!(
                            "{}_servo_{}",
                            self.name,
                            self.servos.len() as i32 + 1
                        )));
                    }

                    if ui.button("Remove servo").clicked() {
                        self.servos.pop();
                    }
                });

                ui.add_space(10.0);

                num_motors = 0;
                egui::Grid::new("Servos grid").show(ui, |ui| {
                    self.servos.iter_mut().enumerate().for_each(|(id, servo)| {
                        num_motors += 1;
                        ui.vertical(|ui| {
                            ui.add_space(20.0);
                            ui.separator();
                            servo.render_options(ui, id + 1000);
                        });

                        if num_motors % num_columns == 0 {
                            ui.end_row();
                        }
                    });
                });
            });
    }
}

impl<
    T: MotorGenerator + PartialEq + PartialOrd + Clone,
    U: ServoGenerator + PartialEq + PartialOrd + Clone,
> SubsystemGenerator for Subsystem<T, U>
{
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl<
    T: MotorGenerator + PartialEq + PartialOrd + Clone,
    U: ServoGenerator + PartialEq + PartialOrd + Clone,
> Subsystem<T, U>
{
    pub fn new(name: String, is_drivetrain: bool) -> Self {
        Subsystem {
            motors: vec![
                T::new(format!("{}_motor_{}", name, 0)),
                T::new(format!("{}_{}", name, 1)),
            ],
            drivetrain_type: DrivetrainType::Mecanum,
            servos: vec![],
            is_drivetrain,
            invert_steering: false,
            name: name.to_string(),
            control_handler: ControlHandler { scripts: vec![], generators: vec![] },
            selected_control: "".to_string(),
        }
    }
}

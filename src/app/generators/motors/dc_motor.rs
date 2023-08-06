use serde::{Deserialize, Serialize};

use strum::IntoEnumIterator;

use super::super::generator;
use super::motor;

use crate::app::generators::{
    self,
    generator::GeneratorSerialize,
    keybinding::keybinding::{Axis, AxisKeybinding, BooleanButton, Keybinding},
    subsystem::subsystem::DrivetrainType,
};

use motor::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct DcMotor {
    pub direction: MotorDirection,
    pub mode: MotorMode,
    pub max_speed: f64,
    pub mecanum_position: MecanumPosition,
    pub arcade_position: ArcadePosition,
    pub name: String,
    pub positions: Vec<Keybinding<i32>>,
    pub speeds_button: Vec<Keybinding<f32>>,
    pub speeds_axis: Vec<AxisKeybinding>,
    pub drivetrain_type: Option<DrivetrainType>,
}

impl GeneratorSerialize for DcMotor {}

impl generator::Generator for DcMotor {
    fn generate_includes(&self) -> String {
        "\
        import com.qualcomm.robotcore.hardware.DcMotor;\n\
        import org.firstinspires.ftc.robotcore.external.Telemetry;\n\
        import com.qualcomm.robotcore.hardware.HardwareMap;\n\
        import com.qualcomm.robotcore.hardware.DcMotorEx;\n\
        import com.qualcomm.robotcore.hardware.DcMotorSimple;\n\n"
            .to_string()
    }

    fn generate_globals(&self) -> String {
        let mut code = format!(
            "\t// {} globals\n\tprivate DcMotorEx {} = null;\n\n",
            &self.name, &self.name
        );

        if self.mode == MotorMode::RUN_TO_POSITION {
            for i in 0..self.positions.len() {
                code += &format!(
                    "\tprivate int {}_pos_{} = {};\n",
                    self.name,
                    i,
                    self.positions.iter().nth(i).unwrap().value
                );
            }
            code += &"\n";
        }

        code
    }

    fn generate_init(&self) -> String {
        format!(
            "\t\t// {} init\n\t\t{} = hardwareMap.get(DcMotorEx.class, \"{}\");\n\n",
            &self.name, &self.name, &self.name
        ) + &format!(
            "\t\t{}.setDirection(DcMotor.Direction.{:?});\n\n",
            &self.name, &self.direction
        ) + &format!(
            "\t\t{}.setMode(DcMotor.RunMode.STOP_AND_RESET_ENCODER);\n",
            &self.name
        ) + &format!("\n\t\t{}.setTargetPosition(0);\n", &self.name)
            + &format!(
            "\t\t{}.setMode(DcMotor.RunMode.{:?});\n\n",
            &self.name, &self.mode
        )
    }

    fn generate_loop(&self) -> String {
        let mut code = match self.drivetrain_type {
            Some(DrivetrainType::Mecanum) => match self.mecanum_position {
                MecanumPosition::FrontLeft =>
                    format!(
                        "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(drive - strafe + turn, -{}, {}));\n\n",
                        &self.name, &self.name, self.max_speed, self.max_speed
                    ),
                MecanumPosition::FrontRight =>
                    format!(
                        "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(drive + strafe - turn, -{}, {}));\n\n",
                        &self.name, &self.name, self.max_speed, self.max_speed
                    ),
                MecanumPosition::RearLeft =>
                    format!(
                        "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(drive + strafe + turn, -{}, {}));\n\n",
                        &self.name, &self.name, self.max_speed, self.max_speed
                    ),
                MecanumPosition::RearRight =>
                    format!(
                        "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(drive - strafe - turn, -{}, {}));\n\n",
                        &self.name, &self.name, self.max_speed, self.max_speed
                    ),
            },
            Some(DrivetrainType::Arcade) => {
                match self.arcade_position {
                    ArcadePosition::Left => {
                        format!(
                            "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(drive + turn, -{}, {}));\n\n",
                            &self.name, &self.name, self.max_speed, self.max_speed
                        )
                    }
                    ArcadePosition::Right => {
                        format!(
                            "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(drive - turn, -{}, {}));\n\n",
                            &self.name, &self.name, self.max_speed, self.max_speed
                        )
                    }
                }
            }
            Some(DrivetrainType::Tank) => {
                match self.arcade_position {
                    ArcadePosition::Left => {
                        format!(
                            "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(driveLeft, -{}, {}));\n\n",
                            &self.name, &self.name, self.max_speed, self.max_speed
                        )
                    }
                    ArcadePosition::Right => {
                        format!(
                            "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(driveRight, -{}, {}));\n\n",
                            &self.name, &self.name, self.max_speed, self.max_speed
                        )
                    }
                }
            }
            None => {
                let mut code = String::new();

                code += &format!("\t\t\t// {} keybindings\n", &self.name);

                for speed_button in &self.speeds_button {
                    if let Some(button) = speed_button.button {
                        code += &format!("\t\t\tif (gamepad1.{:?}) {{\n", button);

                        code += &format!(
                            "\t\t\t\t{}.setPower({:?});\n",
                            &self.name, &speed_button.value);

                        code += &format!("\t\t\t}}\n\n");
                    }
                }

                for speed_axis in &self.speeds_axis {
                    if let Some(axis) = speed_axis.axis {
                        match speed_axis.reversed {
                            true => {
                                code += &format!(
                                    "\t\t\t{}.setPower(gamepad1.{:?} * -1.0f);\n\n",
                                    &self.name, &axis)
                            }
                            false => {
                                code += &format!(
                                    "\t\t\t{}.setPowerTest(gamepad1.{:?});\n\n",
                                    &self.name, &axis)
                            }
                        }
                    }
                }
                code
            }.to_owned(),
        };
        // generate keybindings
        for i in 0..self.positions.len() {
            if self.positions.iter().nth(i).unwrap().button != None {
                code += &format!(
                    "\t\t\tif (gamepad1.{:?}) {{\n",
                    &self.positions.iter().nth(i).unwrap().button.unwrap()
                );

                code += &format!(
                    "\t\t\t\t{}.setTargetPosition({}_pos_{});\n",
                    self.name, self.name, i
                );

                code += &format!(
                    "\t\t\t\t{}.setMode(DcMotor.RunMode.RUN_TO_POSITION);\n",
                    self.name
                );

                code += &format!("\t\t\t\t{}.setVelocity({});\n", self.name, &self.max_speed);

                code += &"\t\t\t}\n\n";
            }
        }
        code
    }

    fn render_options(&mut self, ui: &mut egui::Ui, id: usize) {
        ui.label("DC Motor");
        ui.add_space(10.0);

        ui.text_edit_singleline(&mut self.name);

        ui.push_id(id, |ui| {
            egui::ComboBox::from_label("Run mode")
                .selected_text(format!("{:?}", &mut self.mode))
                .width(170.0)
                .show_ui(ui, |ui| {
                    for mode in MotorMode::iter() {
                        ui.selectable_value(&mut self.mode, mode, format!("{:?}", mode));
                    }
                });
        });

        ui.push_id(id, |ui| {
            egui::ComboBox::from_label("Direction")
                .selected_text(format!("{:?}", &mut self.direction))
                .width(170.0)
                .show_ui(ui, |ui| {
                    for direction in MotorDirection::iter() {
                        ui.selectable_value(
                            &mut self.direction,
                            direction,
                            format!("{:?}", direction),
                        );
                    }
                });
        });

        ui.add_space(20.0);

        match self.mode {
            MotorMode::RUN_TO_POSITION => {
                ui.add(
                    egui::Slider::new(&mut self.max_speed, 0.0..=12000.0)
                        .text("Max speed")
                        .max_decimals(0),
                );
            }
            _ => {
                ui.add(
                    egui::Slider::new(&mut self.max_speed, 0.0..=1.0)
                        .text("Max power")
                        .max_decimals(2),
                );
            }
        }

        ui.add_space(20.0);

        if let Some(drivetrain_type) = self.drivetrain_type {
            if drivetrain_type == DrivetrainType::Mecanum {
                self.render_mecanum(ui, id);
            } else {
                self.render_arcade(ui, id);
            }
        }

        if self.mode == MotorMode::RUN_TO_POSITION {
            self.render_positions(ui, id);
        } else if self.drivetrain_type == None {
            self.render_keybindings(ui, id);
        }
    }
}

impl DcMotor {
    fn render_positions(&mut self, ui: &mut egui::Ui, _id: usize) {
        ui.add_space(10.0);
        ui.label("Fixed positions");

        let mut removed_positions = vec![];

        for (i, pos) in self.positions.iter_mut().enumerate() {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::Separator::default());

                    ui.add(
                        egui::Slider::new(&mut pos.value, 0..=5000)
                            .text("Position")
                            .step_by(1.0)
                            .max_decimals(2),
                    );

                    if ui.button("Delete").clicked() {
                        removed_positions.push(i);
                    }
                });

                let binding_text = match pos.button {
                    Some(bind) => format!("{:?}", bind),
                    None => "None".to_owned(),
                };

                ui.horizontal(|ui| {
                    ui.add(egui::Separator::default());
                    ui.add_space(10.0);

                    egui::ComboBox::new(format!("{}{}", &self.name, &i), "Keybinding")
                        .selected_text(format!("{:?}", binding_text))
                        .width(105.0)
                        .show_ui(ui, |ui| {
                            for button in BooleanButton::iter() {
                                ui.selectable_value(
                                    &mut pos.button,
                                    Some(button),
                                    format!("{:?}", button),
                                );
                            }
                        });
                });
                ui.add_space(10.0);
            });
        }

        for i in removed_positions {
            self.positions.remove(i);
        }

        ui.horizontal(|ui| {
            if ui.button("Add position").clicked() {
                self.positions.push(Keybinding::new(0));
            }
        });
    }

    fn render_keybindings(&mut self, ui: &mut egui::Ui, _id: usize) {
        ui.add_space(10.0);
        ui.label("Keybindings");

        let mut removed_bool_positions = vec![];
        let mut removed_axis_positions = vec![];

        for (i, speed) in self.speeds_button.iter_mut().enumerate() {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::Separator::default());

                    ui.add(
                        egui::Slider::new(&mut speed.value, -1.0..=1.0)
                            .text("Speed")
                            .step_by(0.01)
                            .max_decimals(2),
                    );

                    if ui.button("Delete").clicked() {
                        removed_bool_positions.push(i);
                    }
                });

                let binding_text = match speed.button {
                    Some(bind) => format!("{:?}", bind),
                    None => "None".to_owned(),
                };

                ui.horizontal(|ui| {
                    ui.add(egui::Separator::default());
                    ui.add_space(10.0);

                    egui::ComboBox::new(format!("{}{}", &self.name, &i), "Keybinding")
                        .selected_text(format!("{:?}", binding_text))
                        .width(105.0)
                        .show_ui(ui, |ui| {
                            for button in BooleanButton::iter() {
                                ui.selectable_value(
                                    &mut speed.button,
                                    Some(button),
                                    format!("{:?}", button),
                                );
                            }
                        });
                });

                ui.add_space(10.0);
            });
        }

        for (i, speed) in self.speeds_axis.iter_mut().enumerate() {
            ui.vertical(|ui| {
                let binding_text = match speed.axis {
                    Some(bind) => format!("{:?}", bind),
                    None => "None".to_owned(),
                };

                ui.horizontal(|ui| {
                    ui.add(egui::Separator::default());
                    ui.add_space(10.0);

                    egui::ComboBox::new(format!("{}{}2", &self.name, &i), "Axis")
                        .selected_text(format!("{:?}", binding_text))
                        .width(105.0)
                        .show_ui(ui, |ui| {
                            for button in Axis::iter() {
                                ui.selectable_value(
                                    &mut speed.axis,
                                    Some(button),
                                    format!("{:?}", button),
                                );
                            }
                        });

                    if ui.button("Delete").clicked() {
                        removed_axis_positions.push(i);
                    }
                });

                ui.horizontal(|ui| {
                    ui.add(egui::Separator::default());

                    ui.checkbox(&mut speed.reversed, "Reverse");
                });

                ui.add_space(10.0);
            });
        }

        for i in removed_bool_positions {
            self.speeds_button.remove(i);
        }

        for i in removed_axis_positions {
            self.speeds_axis.remove(i);
        }

        ui.horizontal(|ui| {
            if ui.button("Add keybinding").clicked() {
                self.speeds_button.push(Keybinding::new(0.0));
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Add control axis").clicked() {
                self.speeds_axis.push(AxisKeybinding::new(0.0));
            }
        });
    }

    fn render_mecanum(&mut self, ui: &mut egui::Ui, id: usize) {
        ui.push_id(id + 100, |ui| {
            egui::ComboBox::from_label("Mecanum position")
                .selected_text(format!("{:?}", &mut self.mecanum_position))
                .width(170.0)
                .show_ui(ui, |ui| {
                    for position in MecanumPosition::iter() {
                        ui.selectable_value(
                            &mut self.mecanum_position,
                            position,
                            format!("{:?}", position),
                        );
                    }
                });
        });
    }

    fn render_arcade(&mut self, ui: &mut egui::Ui, id: usize) {
        ui.push_id(id + 100, |ui| {
            egui::ComboBox::from_label(format!("{:?} position", self.drivetrain_type.unwrap()))
                .selected_text(format!("{:?}", &mut self.arcade_position))
                .width(170.0)
                .show_ui(ui, |ui| {
                    for position in ArcadePosition::iter() {
                        ui.selectable_value(
                            &mut self.arcade_position,
                            position,
                            format!("{:?}", position),
                        );
                    }
                });
        });
    }
}

impl Motor for DcMotor {}

impl MotorGenerator for DcMotor {
    fn new(name: String) -> Self {
        DcMotor {
            direction: generators::motors::motor::MotorDirection::FORWARD,
            mode: generators::motors::motor::MotorMode::RUN_USING_ENCODERS,
            max_speed: 1.0,
            mecanum_position: MecanumPosition::FrontLeft,
            arcade_position: ArcadePosition::Left,
            name: name,
            positions: vec![],
            speeds_button: vec![],
            speeds_axis: vec![],
            drivetrain_type: None,
        }
    }

    fn set_drivetrain_type(&mut self, drivetrain_type: Option<DrivetrainType>) {
        self.drivetrain_type = drivetrain_type;
    }

    fn set_mecanum_position(&mut self, position: MecanumPosition) {
        self.mecanum_position = position;
    }
}

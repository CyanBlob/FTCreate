use serde::{Deserialize, Serialize};

use strum::IntoEnumIterator;

use super::super::generator;
use super::motor;

use crate::app::generators::{self, drivetrain::drivetrain::DrivetrainType, motors, generator::GeneratorSerialize};

use motor::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct DcMotor {
    pub direction: motors::motor::MotorDirection,
    pub mode: motors::motor::MotorMode,
    pub max_speed: f64,
    pub mecanum_position: motors::motor::MecanumPosition,
    pub tank_position: motors::motor::TankPosition,
    pub name: String,
    pub positions: Option<Vec<i32>>,
    pub drivetrain_type: Option<DrivetrainType>,
}

impl DcMotor {}

impl GeneratorSerialize for DcMotor {}

impl generator::Generator for DcMotor {
    fn generate_includes(&self) -> String {
        "\
        import com.qualcomm.robotcore.hardware.DcMotor;\n\
        import org.firstinspires.ftc.robotcore.external.Telemetry;\n\
        import com.qualcomm.robotcore.hardware.HardwareMap;\n\
        import com.qualcomm.robotcore.hardware.DcMotorEx;\n\n"
            .to_string()
    }

    fn generate_globals(&self) -> String {
        format!(
            "\t// {} globals\n\tprivate DcMotorEx {} = null;\n\n",
            &self.name, &self.name
        )
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
        ) + &format!(
            "\t\t{}.setMode(DcMotor.RunMode.{:?});\n\n",
            &self.name, &self.mode
        )
    }

    fn generate_loop(&self) -> String {
        match self.drivetrain_type {
        Some(DrivetrainType::Mecanum) => match self.mecanum_position {
            MecanumPosition::FrontLeft => 
                format!(
                    "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(  drive - strafe + turn, -{}, {}));\n\n",
                    &self.name, &self.name, self.max_speed, self.max_speed
                ),
            MecanumPosition::FrontRight => 
                format!(
                    "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(  drive + strafe - turn, -{}, {}));\n\n",
                    &self.name, &self.name, self.max_speed, self.max_speed
                ),
            MecanumPosition::RearLeft =>
                format!(
                    "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(  drive + strafe + turn, -{}, {}));\n\n",
                    &self.name, &self.name, self.max_speed, self.max_speed
                ),
            MecanumPosition::RearRight => 
                format!(
                    "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(  drive - strafe - turn, -{}, {}));\n\n",
                    &self.name, &self.name, self.max_speed, self.max_speed
                ),
        }
            Some(DrivetrainType::Tank) => {
                match self.tank_position {
                    TankPosition::Left => {
                format!(
                    "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(  drive + turn, -{}, {}));\n\n",
                    &self.name, &self.name, self.max_speed, self.max_speed
                )},
                    TankPosition::Right => {
                format!(
                    "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(  drive - turn, -{}, {}));\n\n",
                    &self.name, &self.name, self.max_speed, self.max_speed
                )},
                    }
                }
            None => "".to_string(),
    }
    }

    fn render_options(&mut self, ui: &mut egui::Ui, id: usize) {
        let max_speed = 1.0;

        ui.label("DC Motor");
        ui.add_space(10.0);

        ui.text_edit_singleline(&mut self.name);

        // TODO: refactor into function
        ui.push_id(id, |ui| {
            egui::ComboBox::from_label("Run mode")
                .selected_text(format!("{:?}", &mut self.mode))
                .width(170.0)
                .show_ui(ui, |ui| {
                    for mode in motor::MotorMode::iter() {
                        ui.selectable_value(&mut self.mode, mode, format!("{:?}", mode));
                    }
                });
        });

        ui.push_id(id, |ui| {
            egui::ComboBox::from_label("Direction")
                .selected_text(format!("{:?}", &mut self.direction))
                .width(170.0)
                .show_ui(ui, |ui| {
                    for direction in motor::MotorDirection::iter() {
                        ui.selectable_value(
                            &mut self.direction,
                            direction,
                            format!("{:?}", direction),
                        );
                    }
                });
        });

        ui.add_space(20.0);

        ui.add(egui::Slider::new(&mut self.max_speed, 0.0..=max_speed).text("Max speed"));

        ui.add_space(20.0);
        
        if let Some(drivetrain_type) = self.drivetrain_type {
            if drivetrain_type == DrivetrainType::Mecanum {
                ui.push_id(id + 100, |ui| {
                    egui::ComboBox::from_label("Mecanum position")
                        .selected_text(format!("{:?}", &mut self.mecanum_position))
                        .width(170.0)
                        .show_ui(ui, |ui| {
                            for position in MecanumPosition::iter() {
                                ui.selectable_value(&mut self.mecanum_position, position, format!("{:?}", position));
                            }
                        });
                });
            }

            else if drivetrain_type == DrivetrainType::Tank {
                ui.push_id(id + 100, |ui| {
                    egui::ComboBox::from_label("Tank position")
                        .selected_text(format!("{:?}", &mut self.tank_position))
                        .width(170.0)
                        .show_ui(ui, |ui| {
                            for position in TankPosition::iter() {
                                ui.selectable_value(&mut self.tank_position, position, format!("{:?}", position));
                            }
                        });
                });
            }
        }

    }
}

impl Motor for DcMotor {}

impl MotorGenerator for DcMotor {
    fn new() -> Self {
        DcMotor {
            direction: generators::motors::motor::MotorDirection::FORWARD,
            mode: generators::motors::motor::MotorMode::RUN_TO_POSITION,
            max_speed: 1.0,
            mecanum_position: MecanumPosition::FrontLeft,
            tank_position: TankPosition::Left,
            name: "Motor".to_string(),
            positions: None,
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

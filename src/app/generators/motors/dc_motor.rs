use serde::{Deserialize, Serialize};

use strum::IntoEnumIterator;

use super::super::generator;
use super::motor;

use crate::app::generators::{motors, self};

use motor::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct DcMotor {
    pub direction: motors::motor::MotorDirection,
    pub mode: motors::motor::MotorMode,
    pub max_speed: f64,
    pub position: u8,
    pub name: String,
}

impl DcMotor {
}

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
        format!(
            "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(  drive - strafe + turn, -{}, {}));\n\n",
            &self.name, &self.name, self.max_speed, self.max_speed
        )
    }

    fn render_options(&mut self, ui: &mut egui::Ui, id: usize) {
        let max_speed = 1.0;

        ui.label("DC Motor");
        ui.add_space(10.0);

        ui.text_edit_singleline(&mut self.name);

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

        if ui.button("Increment").clicked() {
            self.max_speed += 0.1;

            if self.max_speed > max_speed {
                self.max_speed = max_speed;
            }
        }

        if ui.button("Decrement").clicked() {
            self.max_speed -= 0.1;

            if self.max_speed < 0.0 {
                self.max_speed = 0.0;
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
            position: 0,
            name: "Motor".to_string(),
        }
    }
}

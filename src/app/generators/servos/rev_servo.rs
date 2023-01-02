use serde::{Deserialize, Serialize};

use strum::IntoEnumIterator;

use super::super::generator;
use super::servo;

use crate::app::generators::{servos, self};

use servo::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct RevServo {
    pub direction: servos::servo::ServoDirection,
    pub mode: servos::servo::ServoMode,
    pub name: String,
}

impl RevServo {
}

impl generator::Generator for RevServo {
    fn generate_includes(&self) -> String {
        "\
        import com.qualcomm.robotcore.hardware.RevServo;\n\
        import org.firstinspires.ftc.robotcore.external.Telemetry;\n\
        import com.qualcomm.robotcore.hardware.HardwareMap;\n\
        import com.qualcomm.robotcore.hardware.RevServoEx;\n\n"
            .to_string()
    }

    fn generate_globals(&self) -> String {
        format!(
            "\t// {} globals\n\tprivate RevServoEx {} = null;\n\n",
            &self.name, &self.name
        )
    }

    fn generate_init(&self) -> String {
        format!(
            "\t\t// {} init\n\t\t{} = hardwareMap.get(CRServo.class, \"{}\");\n\n",
            &self.name, &self.name, &self.name
        ) + &format!(
            "\t\t{}.setDirection(RevServo.Direction.{:?});\n\n",
            &self.name, &self.direction
        )
    }

    /*fn generate_loop(&self) -> String {
        format!(
            "\t\t\t// {} loop\n\t\t\t{}.setPower(Range.clip(  drive - strafe + turn, -{}, {}));\n\n",
            &self.name, &self.name, self.max_speed, self.max_speed
        )
    }*/

    fn render_options(&mut self, ui: &mut egui::Ui, id: usize) {
        ui.label("Rev servo");
        ui.add_space(10.0);

        ui.text_edit_singleline(&mut self.name);

        ui.push_id(id, |ui| {
            egui::ComboBox::from_label("Servo mode")
                .selected_text(format!("{:?}", &mut self.mode))
                .width(170.0)
                .show_ui(ui, |ui| {
                    for mode in servo::ServoMode::iter() {
                        ui.selectable_value(&mut self.mode, mode, format!("{:?}", mode));
                    }
                });
        });

        ui.push_id(id, |ui| {
            egui::ComboBox::from_label("Direction")
                .selected_text(format!("{:?}", &mut self.direction))
                .width(170.0)
                .show_ui(ui, |ui| {
                    for direction in servo::ServoDirection::iter() {
                        ui.selectable_value(
                            &mut self.direction,
                            direction,
                            format!("{:?}", direction),
                        );
                    }
                });
        });

        ui.add_space(20.0);

    }
}

impl Servo for RevServo {}

impl ServoGenerator for RevServo {
    fn new() -> Self {
        RevServo {
            direction: generators::servos::servo::ServoDirection::FORWARD,
            mode: generators::servos::servo::ServoMode::Servo,
            name: "Servo".to_string(),
        }
    }
}

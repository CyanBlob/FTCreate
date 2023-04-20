use serde::{Deserialize, Serialize};

use strum::IntoEnumIterator;

use super::super::generator;
use super::servo;

use crate::app::generators::{
    self,
    generator::GeneratorSerialize,
    keybinding::keybinding::{BooleanButton, Keybinding},
    servos,
};

use servo::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct RevServo {
    pub direction: servos::servo::ServoDirection,
    pub mode: servos::servo::ServoMode,
    pub name: String,
    pub positions: Vec<Keybinding<f32>>,
}

impl generator::Generator for RevServo {
    fn generate_includes(&self) -> String {
        "\
        import com.qualcomm.robotcore.hardware.CRServo;\n\
        import org.firstinspires.ftc.robotcore.external.Telemetry;\n\
        import com.qualcomm.robotcore.hardware.HardwareMap;\n\n"
            .to_string()
    }

    fn generate_globals(&self) -> String {
        let mut code = format!(
            "\t// {} globals\n\tprivate CRServo {} = null;\n\n",
            &self.name, &self.name
        );

        for i in 0..self.positions.len() {
            code += &format!(
                "\tprivate double {}_pos_{} = {};\n",
                self.name,
                i,
                self.positions.iter().nth(i).unwrap().value
            );
        }
        code += &"\n";
        code
    }

    fn generate_init(&self) -> String {
        format!(
            "\t\t// {} init\n\t\t{} = hardwareMap.get(CRServo.class, \"{}\");\n\n",
            &self.name, &self.name, &self.name
        ) + &format!(
            "\t\t{}.setDirection(CRServo.Direction.{:?});\n\n",
            &self.name, &self.direction
        )
    }

    fn generate_loop(&self) -> String {
        let mut code: String = "".into();
        // generate keybindings
        for i in 0..self.positions.len() {
            if self.positions.iter().nth(i).unwrap().button != None {
                code += &format!(
                    "\t\t\tif (gamepad1.{:?}) {{\n",
                    &self.positions.iter().nth(i).unwrap().button.unwrap()
                );

                code += &format!("\t\t\t\t{}.setPower({}_pos_{});\n", self.name, self.name, i);

                code += "\t\t\t}\n\n";
            }
        }
        code
    }

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

        self.render_positions(ui, id);

        ui.add_space(20.0);
    }
}

impl RevServo {
    fn render_positions(&mut self, ui: &mut egui::Ui, _id: usize) {
        ui.add_space(10.0);
        ui.label("Fixed positions");

        let mut removed_positions = vec![];

        for (i, pos) in self.positions.iter_mut().enumerate() {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::Separator::default());

                    ui.add(
                        egui::Slider::new(&mut pos.value, -1.0..=1.0)
                            .text("Position")
                            .step_by(0.01)
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
                self.positions.push(Keybinding::new(0.0));
            }
        });
    }
}

impl GeneratorSerialize for RevServo {}

impl Servo for RevServo {}

impl ServoGenerator for RevServo {
    fn new(name: String) -> Self {
        RevServo {
            direction: generators::servos::servo::ServoDirection::FORWARD,
            mode: generators::servos::servo::ServoMode::Servo,
            name: name,
            positions: vec![],
        }
    }
}

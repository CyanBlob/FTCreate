use serde::{Deserialize, Serialize};

use strum::IntoEnumIterator;

use super::super::generator;
use super::servo;

use crate::app::generators::{
    generator::GeneratorSerialize,
    keybinding::keybinding::{BooleanButton, Keybinding},
};

use servo::*;
use crate::app::generators::keybinding::keybinding::BooleanButton::default;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct RevServo {
    pub direction: ServoDirection,
    pub mode: ServoMode,
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

        let mut _positions = self.positions.clone();

        _positions.sort();

        // generate keybindings
        let mut count = 0;
        let mut default_code: String = "".into();
        for speed_button in _positions {
            if let Some(button) = speed_button.button {
                if button == default {
                    default_code = format!("\t\t\telse {{\n\t\t\t\t{}.setPower({});\n\t\t\t}}\n\n", &self.name, &speed_button.value);
                    continue;
                }

                code += "\t\t\t";

                if count > 0 {
                    code += "else "
                }

                code += &format!(
                    "if (gamepad1.{:?}) {{\n",
                    &button
                );

                code += &format!("\t\t\t\t{}.setPower({});\n", self.name, &speed_button.value);

                code += "\t\t\t}\n\n";

                count += 1;
            }
        }
        code += &default_code;

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
                    for mode in ServoMode::iter() {
                        ui.selectable_value(&mut self.mode, mode, format!("{:?}", mode));
                    }
                });
        });

        ui.push_id(id, |ui| {
            egui::ComboBox::from_label("Direction")
                .selected_text(format!("{:?}", &mut self.direction))
                .width(170.0)
                .show_ui(ui, |ui| {
                    for direction in ServoDirection::iter() {
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
            direction: ServoDirection::FORWARD,
            mode: ServoMode::Servo,
            name: name,
            positions: vec![],
        }
    }
}

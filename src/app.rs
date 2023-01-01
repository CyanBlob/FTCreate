use crate::app::generators::generator::Generator;
pub mod generators;

use generators::drivetrain;
use generators::motors::motor::MotorGenerator;

use crate::app::drivetrain::drivetrain::Drivetrain;
use generators::motors::dc_motor::DcMotor;

pub mod syntax_highlighting;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    drivetrain: Drivetrain<DcMotor>,
    code: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            drivetrain: //Rc::new(
            Drivetrain {
                motors: //Rc::new(
                vec![DcMotor {
                    direction: generators::motors::motor::MotorDirection::FORWARD,
                    mode: generators::motors::motor::MotorMode::RUN_TO_POSITION,
                    max_speed: 0.75,
                    position: 0,
                    name: "Motor1".to_string()
                },
                    DcMotor {
                        direction: generators::motors::motor::MotorDirection::REVERSE,
                        mode: generators::motors::motor::MotorMode::RUN_WITHOUT_ENCODERS,
                        max_speed: -1.0,
                        position: 0,
                        name: "Motor2".to_string()
                    }
                ],
                //),
            },
            code: "Click \"GENERATE!\"".to_string(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn generate_code(&mut self) {
        let mut new_code = String::new();
        
        new_code += "package org.firstinspires.ftc.teamcode;\n\n";

        // includes
        new_code += &self
            .drivetrain
            .motors
            .iter_mut()
            .nth(0)
            .unwrap()
            .generate_includes();
        
            new_code += r#"@TeleOp(name="EasyFTC Teleop", group="Linear Opmode")"#;
            new_code += "\n";
            new_code += "public class EasyFTC_teleop extends LinearOpMode {\n\
                \tprivate ElapsedTime runtime = new ElapsedTime();\n\n";

        // globals
        self.drivetrain
            .motors
            .iter_mut()
            .for_each(|motor| new_code += &motor.generate_globals());
        
        new_code += "\n\t@Override\n\
        \tpublic void runOpMode() {\n\n\
            \t\ttelemetry.addData(\"Status\", \"Initialized\");\n\
            \t\ttelemetry.update();";
        new_code += "\n\n";

        // initializers
        self.drivetrain
            .motors
            .iter_mut()
            .for_each(|motor| new_code += &motor.generate_init());
        
        new_code += "\n\t\twaitForStart();\n\n\
            \t\t// Reset the timer (stopwatch) because we only care about time since the game\n\
            \t\t// actually starts\n\
            \t\truntime.reset();\n\n\
            \t\twhile (opModeIsActive()) {\n\n";
        
        // loop one-time setup
        new_code += &self
            .drivetrain
            .motors
            .iter_mut()
            .nth(0)
            .unwrap()
            .generate_loop_one_time_setup();

        // loop
        self.drivetrain
            .motors
            .iter_mut()
            .for_each(|motor| new_code += &motor.generate_loop());
        
        new_code += "\t\t\ttelemetry.update();\n\
                \t\t}\n\
            \t}";

        self.code = new_code.to_string();
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, value, .. } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        /*egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });*/

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Drivetrain Configuration");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);

            self.drivetrain
                .motors
                .iter_mut()
                .enumerate()
                .for_each(|(id, motor)| {
                    ui.add_space(20.0);
                    ui.separator();
                    motor.render_options(ui, id);
                });
        });

        egui::SidePanel::right("code_panel").show(ctx, |ui| {
            ui.heading("Generated code");

            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                //ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.code.to_string()));
                show_code(ui, &self.code);
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if ui.button("GENERATE!").clicked() {
                    self.generate_code();
                }
            });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}

fn show_code(ui: &mut egui::Ui, code: &str) {
    let code = remove_leading_indentation(code.trim_start_matches('\n'));
    crate::app::syntax_highlighting::code_view_ui(ui, &code);
}

fn remove_leading_indentation(code: &str) -> String {
    fn is_indent(c: &u8) -> bool {
        matches!(*c, b' ' | b'\t')
    }

    let first_line_indent = code.bytes().take_while(is_indent).count();

    let mut out = String::new();

    let mut code = code;
    while !code.is_empty() {
        let indent = code.bytes().take_while(is_indent).count();
        let start = first_line_indent.min(indent);
        let end = code
            .find('\n')
            .map_or_else(|| code.len(), |endline| endline + 1);
        out += &code[start..end];
        code = &code[end..];
    }
    out
}

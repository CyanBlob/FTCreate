use crate::app::generators::generator::Generator;
pub mod generators;

use generators::drivetrain;

use crate::app::drivetrain::drivetrain::Drivetrain;
use generators::motors::dc_motor::DcMotor;
use generators::servos::rev_servo::RevServo;

use self::generators::{drivetrain::drivetrain::DrivetrainType, motors::motor::{MecanumPosition, TankPosition}};

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

    drivetrain: Drivetrain<DcMotor, RevServo>,
    code: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            drivetrain: Drivetrain {
                motors: vec![
                    DcMotor {
                        direction: generators::motors::motor::MotorDirection::FORWARD,
                        mode: generators::motors::motor::MotorMode::RUN_TO_POSITION,
                        max_speed: 0.75,
                        mecanum_position: MecanumPosition::FrontLeft,
                        tank_position: TankPosition::Left,
                        name: "Motor1".to_string(),
                        drivetrain_type: DrivetrainType::Mecanum,
                        positions: None,
                    },
                    DcMotor {
                        direction: generators::motors::motor::MotorDirection::REVERSE,
                        mode: generators::motors::motor::MotorMode::RUN_WITHOUT_ENCODERS,
                        max_speed: 1.0,
                        mecanum_position: MecanumPosition::FrontRight,
                        tank_position: TankPosition::Right,
                        name: "Motor2".to_string(),
                        drivetrain_type: DrivetrainType::Mecanum,
                        positions: None,
                    },
                ],
                drivetrain_type: DrivetrainType::Mecanum,
                servos: vec![]
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
        if self.drivetrain.motors.len() > 0 as usize {
            new_code += &self
                .drivetrain
                .motors
                .iter_mut()
                .nth(0)
                .unwrap()
                .generate_includes();
        }

        new_code += r#"@TeleOp(name="EasyFTC Teleop", group="Linear Opmode")"#;
        new_code += "\n";
        new_code += "public class EasyFTC_teleop extends LinearOpMode {\n\
                \n\tprivate ElapsedTime runtime = new ElapsedTime();\n\n";

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
        new_code += &self.drivetrain.generate_loop_one_time_setup();

        if self.drivetrain.motors.len() > 0 as usize {
            new_code += &self
                .drivetrain
                .motors
                .iter_mut()
                .nth(0)
                .unwrap()
                .generate_loop_one_time_setup();
        }

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

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Drivetrain Configuration");
            egui::warn_if_debug_build(ui);

            self.drivetrain.render_options(ui, 0);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                if ui.button("GENERATE!").clicked() {
                    self.generate_code();
                }
            });
        });

        egui::SidePanel::right("code_panel").show(ctx, |ui| {
            ui.heading("Generated code");

            egui::scroll_area::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        show_code(ui, &self.code);
                    });
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

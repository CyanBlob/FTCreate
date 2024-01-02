use std::fs;
use crate::app::generators::generator::Generator;

pub mod generators;

use generators::motors::dc_motor::DcMotor;
use generators::servos::rev_servo::RevServo;

use self::generators::generator::SubsystemGenerator;
use self::generators::subsystem::subsystem::Subsystem;
use self::theme::Theme;

use std::fs::File;
use std::io::{Write};
use std::path::PathBuf;
use mlua::Lua;
use tokio::sync::{mpsc, mpsc::unbounded_channel};

#[cfg(not(target_arch = "wasm32"))]
use tokio;
#[cfg(not(target_arch = "wasm32"))]
use tokio::runtime::Runtime;
use crate::app::generators::lua_generator::{ControlHandler, LuaGenerator};

pub mod syntax_highlighting;

pub mod theme;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    file_name: String,

    drivetrain: Subsystem<DcMotor, RevServo>,
    subsystems: Vec<Subsystem<DcMotor, RevServo>>,
    code: String,

    #[serde(skip)]
    selected_subsystem: usize,

    #[serde(skip)]
    upload_status_tx: mpsc::UnboundedSender<UploadStatus>,
    #[serde(skip)]
    upload_status_rx: mpsc::UnboundedReceiver<UploadStatus>,
    upload_status: String,
    last_upload_update: UploadStatus,

    #[serde(skip)]
    #[cfg(not(target_arch = "wasm32"))]
    tokio_runtime: Runtime,

    #[serde(skip)]
    lua: Lua,
    control_handler: ControlHandler,
    lua_scripts: Vec::<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[allow(non_camel_case_types)]
enum UploadStatus {
    DISCONNECTED,
    CONNECTING,
    CONNECTED,
    CONNECT_FAILED,
    UPLOADING,
    UPLOADED,
    UPLOAD_FAILED,
    BUILDING,
    BUILT,
    BUILD_FAILED,
}

unsafe impl Send for UploadStatus {}

unsafe impl Sync for UploadStatus {}

impl Default for TemplateApp {
    fn default() -> Self {
        let (tx, rx) = unbounded_channel::<UploadStatus>();
        let _ = tx.send(UploadStatus::DISCONNECTED);

        Self {
            label: "FTCreate".to_owned(),
            file_name: "FTCreate".to_owned(),
            drivetrain: Subsystem::new("Drivetrain".to_owned(), true),
            subsystems: vec![],
            code: "".to_string(),
            selected_subsystem: 0,
            upload_status_tx: tx,
            upload_status_rx: rx,
            upload_status: "Disconnected".into(),
            last_upload_update: UploadStatus::DISCONNECTED,
            #[cfg(not(target_arch = "wasm32"))]
            tokio_runtime: Runtime::new().unwrap(),
            lua: Lua::new(),
            control_handler: ControlHandler { generators: vec![] },
            lua_scripts: vec![],
        }
    }
}

impl TemplateApp {
    // Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        //cc.egui_ctx.set_visuals(egui::Visuals::light());

        let theme: Theme = Theme::new(&crate::config::AppStyle::default());
        cc.egui_ctx.set_visuals(theme.visuals);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut obj: TemplateApp = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            for generator in &mut obj.control_handler.generators {
                generator.load();
            }

            return obj;
        }

        let mut obj: TemplateApp = Default::default();

        let paths = fs::read_dir("./lua_modules").unwrap();

        for path in paths {
            obj.lua_scripts.push(path.unwrap().path().to_str().unwrap().to_string());
        }

        for script in &obj.lua_scripts {
            println!("Loading: {:?}", script);
            obj.control_handler.generators.push(LuaGenerator::new(script));
        }

        obj
    }

    pub fn generate_code(&mut self) {
        let mut new_code = String::new();

        // standard includes
        new_code += "package org.firstinspires.ftc.teamcode;\n\n";
        new_code += "import com.qualcomm.robotcore.eventloop.opmode.LinearOpMode;\n";
        new_code += "import com.qualcomm.robotcore.eventloop.opmode.TeleOp;\n";
        new_code += "import com.qualcomm.robotcore.util.ElapsedTime;\n";
        new_code += "import com.qualcomm.robotcore.util.Range;\n";

        // subsystem includes
        let mut includes = self.drivetrain.generate_includes().to_string();

        self.subsystems.iter().for_each(|subsystem| {
            includes += &subsystem.generate_includes().to_string();
        });

        let mut lua_includes = self.control_handler.generate_includes();
        lua_includes = lua_includes.to_string().split("\n").map(|line| {format!("\n{}", line)}).collect::<String>();
        new_code += &lua_includes;

        // remove duplicate includes
        let mut includes_collection = includes.lines().collect::<Vec<&str>>();
        includes_collection.sort();
        includes_collection.dedup();

        for include in includes_collection {
            new_code += &include;
            new_code += "\n";
        }

        new_code += "\n";

        new_code += &format!(
            r#"@TeleOp(name="{} Teleop", group="Linear Opmode")"#,
            &self.file_name
        );
        new_code += "\n";
        new_code += &format!(
            "public class {} extends LinearOpMode {{\n\
                \n\tprivate ElapsedTime runtime = new ElapsedTime();\n\n",
            &self.file_name
        );

        // global variables
        new_code += &self.drivetrain.generate_globals();

        self.subsystems.iter().for_each(|subsystem| {
            new_code += &subsystem.generate_globals();
        });

        let mut lua_globals = self.control_handler.generate_globals();
        lua_globals = lua_globals.to_string().split("\n").map(|line| {format!("\n\t{}", line)}).collect::<String>();
        new_code += &lua_globals;

        new_code += "\n\t@Override\n\
        \tpublic void runOpMode() {\n\n\
            \t\ttelemetry.addData(\"Status\", \"Initialized\");\n\
            \t\ttelemetry.update();";
        new_code += "\n\n";

        // initializers
        new_code += &self.drivetrain.generate_init();

        self.subsystems.iter().for_each(|subsystem| {
            new_code += &subsystem.generate_init();
        });

        let mut lua_init = self.control_handler.generate_init();
        lua_init = lua_init.to_string().split("\n").map(|line| {format!("\n\t\t{}", line)}).collect::<String>();
        new_code += &lua_init;

        new_code += "\n\t\twaitForStart();\n\n\
            \t\t// Reset the timer (stopwatch) because we only care about time since the game\n\
            \t\t// actually starts\n\
            \t\truntime.reset();\n\n\
            \t\twhile (opModeIsActive()) {\n\n";

        // loop one-time setup
        new_code += &self.drivetrain.generate_loop_one_time_setup();

        self.subsystems.iter().for_each(|subsystem| {
            new_code += &subsystem.generate_loop_one_time_setup();
        });

        let mut lua_one_time_setup = self.control_handler.generate_loop_one_time_setup();
        lua_one_time_setup = lua_one_time_setup.to_string().split("\n").map(|line| {format!("\n\t\t\t{}", line)}).collect::<String>();
        new_code += &lua_one_time_setup;

        new_code += "\n";

        // loop
        new_code += &self.drivetrain.generate_loop();

        self.subsystems.iter().for_each(|subsystem| {
            new_code += &subsystem.generate_loop();
        });

        let mut lua_loop =  self.control_handler.generate_loop();
        lua_loop = lua_loop.to_string().split("\n").map(|line| {format!("\n\t\t\t{}", line)}).collect::<String>();
        new_code += &lua_loop;

        new_code += "\n\t\t\ttelemetry.update();\n\
                \t\t}\n\
            \t}\n}";

        self.code = new_code.to_string();
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // periodically update the UI without user interaction so build status can update
        match self.last_upload_update {
            UploadStatus::DISCONNECTED | UploadStatus::UPLOAD_FAILED | UploadStatus::CONNECT_FAILED | UploadStatus::BUILD_FAILED => {
                ctx.request_repaint_after(std::time::Duration::from_secs(60));
            }
            _ => {
                ctx.request_repaint_after(std::time::Duration::from_secs(1));
            }
        }

        egui::SidePanel::right("code_panel").show(ctx, |ui| {
            self.control_handler.render(ui);

            ui.heading("Generated code");
            egui::scroll_area::ScrollArea::horizontal().show(ui, |ui| {
                egui::scroll_area::ScrollArea::vertical()
                    .auto_shrink([true; 2])
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                            show_code(ui, &self.code, ui.available_width());
                        });
                    });
            });
        });

        #[cfg(not(target_arch = "wasm32"))]
        egui::TopBottomPanel::bottom("Upload").show(ctx, |ui| {
            {
                match &self.upload_status_rx.try_recv() {
                    Ok(status) => {
                        self.last_upload_update = status.clone();

                        self.upload_status = match status {
                            UploadStatus::DISCONNECTED => "Not connected to robot".into(),
                            UploadStatus::CONNECTING => "Connecting to robot".into(),
                            UploadStatus::CONNECTED => "Connection successful".into(),
                            UploadStatus::CONNECT_FAILED => {
                                "Connection failed. Ensure you're on the robot's WiFi network"
                                    .into()
                            }
                            UploadStatus::UPLOADING => "Uploading code to robot".into(),
                            UploadStatus::UPLOADED => "Uploaded code to robot".into(),
                            UploadStatus::UPLOAD_FAILED => "Failed to upload code".into(),
                            UploadStatus::BUILDING => "Building code...".into(),
                            UploadStatus::BUILT => "Code built successfully. Ready to run".into(),
                            UploadStatus::BUILD_FAILED => "Build failed".into(),
                        };
                    }
                    Err(_) => {}
                };
            }

            ui.label(&self.upload_status);

            if ui.button("Upload code").clicked() {
                let code = self.code.clone();
                let tx = self.upload_status_tx.clone();
                let file_name = self.file_name.clone();

                self.tokio_runtime.spawn(async move {
                    upload_code(code, &tx, file_name.to_owned()).await;
                });
            }

            if ui.button("Reload lua modules").clicked() {

                self.lua_scripts.clear();
                self.control_handler.generators.clear();

                let paths = fs::read_dir("./lua_modules").unwrap();

                for path in paths {
                    self.lua_scripts.push(path.unwrap().path().to_str().unwrap().to_string());
                }

                for script in &self.lua_scripts {
                    println!("Loading: {:?}", script);
                    self.control_handler.generators.push(LuaGenerator::new(script));
                }
            }
        });

        #[cfg(target_arch = "wasm32")]
        egui::TopBottomPanel::bottom("Upload").show(ctx, |ui| {
            ui.label("Code upload only works from desktop version of FTCreate: ");
            ui.hyperlink("https://github.com/FRC3005/FTCreate/releases")
                .on_hover_text("Download page");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("subsystem_panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Subsystems: ");
                    if ui.button("Drivetrain").clicked() {
                        self.selected_subsystem = 0;
                    }

                    self.subsystems
                        .iter()
                        .enumerate()
                        .for_each(|(i, subsystem)| {
                            if ui.button(subsystem.get_name()).clicked() {
                                self.selected_subsystem = i + 1;
                            }
                        });

                    if ui.button("Add subsystem").clicked() {
                        self.subsystems.push(Subsystem::new(
                            format!("Subsystem_{}", self.subsystems.len() as i32 + 1),
                            false,
                        ));
                        self.selected_subsystem = self.subsystems.len();
                    }
                });
            });

            ui.add_space(30.0);

            if self.selected_subsystem == 0 {
                ui.horizontal(|ui| {
                    ui.label("Teleop name: ");
                    ui.text_edit_singleline(&mut self.file_name);
                });

                ui.add_space(10.0);

                ui.heading("Drivetrain Configuration");
            } else {
                ui.heading(format!(
                    "{} Configuration",
                    self.subsystems
                        .iter()
                        .nth(self.selected_subsystem - 1)
                        .unwrap()
                        .get_name()
                ));

                ui.horizontal(|ui| {
                    let text_edit = egui::TextEdit::singleline(
                        &mut self
                            .subsystems
                            .iter_mut()
                            .nth(self.selected_subsystem - 1)
                            .unwrap()
                            .name,
                    )
                        .desired_width(100.0);
                    ui.add(text_edit);
                    ui.label("Rename subsystem");
                });

                ui.add_space(10.0);

                if ui.button("Delete subsystem").clicked() {
                    self.subsystems.remove(self.selected_subsystem - 1);
                    self.selected_subsystem -= 1;
                }
            }
            if self.selected_subsystem == 0 {
                self.drivetrain.render_options(ui, 0);
            } else {
                self.subsystems
                    .iter_mut()
                    .nth(self.selected_subsystem - 1)
                    .unwrap()
                    .render_options(ui, 0);
            }

            self.generate_code();
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn show_code(ui: &mut egui::Ui, code: &str, width: f32) {
    let code = remove_leading_indentation(code.trim_start_matches('\n'));
    syntax_highlighting::code_view_ui(ui, &code, width);
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

#[cfg(not(target_arch = "wasm32"))]
async fn upload_code(
    code: String,
    upload_status_tx: &mpsc::UnboundedSender<UploadStatus>,
    file_name: String,
) {
    let mut opt: ftc_http::Ftc = ftc_http::Ftc::default();
    opt.upload = true;

    let _ = upload_status_tx.send(UploadStatus::CONNECTING);

    let mut conf = ftc_http::AppConfig::default();

    match ftc_http::RobotController::new(&mut conf).await {
        Ok(r) => {
            let _ = upload_status_tx.send(UploadStatus::CONNECTED);

            // create a tmp directory to write files into
            let dir = tempfile::tempdir().unwrap();

            // write teleop to a file
            let file_path = dir.path().join(&format!("{}.java", &file_name));
            let mut tmpfile = File::create(&file_path).unwrap();

            write!(tmpfile, "{}", code).unwrap();

            let _ = upload_status_tx.send(UploadStatus::UPLOADING);
            println!("Uploading files...");
            match r
                .upload_files(
                    vec![PathBuf::from(&file_path)],
                    &format!("{}.java", &file_name),
                )
                .await
            {
                Ok(_) => {
                    let _ = upload_status_tx.send(UploadStatus::BUILDING);
                    match r.build().await {
                        Ok(_) => {
                            let _ = upload_status_tx.send(UploadStatus::BUILT);
                            println!("Build succeeded");
                        }
                        Err(_) => {
                            let _ = upload_status_tx.send(UploadStatus::BUILD_FAILED);
                            println!("Build failed");
                        }
                    }
                }
                Err(_) => {
                    let _ = upload_status_tx.send(UploadStatus::UPLOAD_FAILED);
                    println!("Failed to upload files to robot");
                }
            }
        }
        Err(_) => {
            let _ = upload_status_tx.send(UploadStatus::CONNECT_FAILED);
        }
    };
}

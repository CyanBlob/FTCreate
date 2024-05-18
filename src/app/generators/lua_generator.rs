use crate::app::generators::control::Control;
use crate::app::generators::ui_elements::{
    ButtonInput, CheckboxInput, ComboBoxInput, Slider, TextInput,
};
use egui::Ui;
use mlua::prelude::LuaError;
use mlua::{Function, Lua, Table, UserData, UserDataMethods};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ControlHandler {
    pub scripts: Vec<String>,
    pub generators: Vec<LuaGenerator>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct LuaGenerator {
    #[serde(skip)]
    pub lua: Lua,
    pub script: String,
    pub script_data: String,
    pub controls: Vec<Control>,
    #[serde(skip)]
    pub loaded: bool,
}

impl Clone for LuaGenerator {
    fn clone(&self) -> Self {
        return Self {
            lua: Lua::new(),
            script: self.script.clone(),
            script_data: self.script_data.clone(),
            controls: vec![],
            loaded: false,
        };
    }
}

impl ControlHandler {
    pub fn render(&mut self, ui: &mut Ui) {
        self.tick_lua();

        self.add_controls();

        self.render_controls(ui);
    }

    pub fn add_controls(&mut self) {
        let mut i = 0;
        for generator in &mut self.generators {
            let mut new_controls = vec![];

            let get_controls: Function<'_> = generator
                .lua
                .globals()
                .get("get_controls")
                .expect("Lua scripts must contain a 'get_controls' method!");
            let controls_changed: Function<'_> = generator
                .lua
                .globals()
                .get("controls_changed")
                .expect("Lua scripts must contain a 'controls_changed' method!");

            let changed = controls_changed.call::<_, bool>(()).unwrap();

            if changed == false {
                // ensure unique IDs for each control. This is needed for repeated ComboBox controls
                i += 100;
                continue;
            }

            generator.controls.clear();

            let table = get_controls.call::<_, Table<'_>>(()).unwrap();

            table
                .for_each(|k: String, v: Table<'_>| {
                    i += 1;
                    match v.raw_get::<i32, String>(1) {
                        Ok(s) => match s.as_str() {
                            "Slider" => {
                                let control = Control::SliderType(Slider {
                                    name: v.raw_get::<i32, String>(2).unwrap(),
                                    label: v.raw_get::<i32, String>(3).unwrap(),
                                    min: v.raw_get::<i32, f32>(4).unwrap(),
                                    max: v.raw_get::<i32, f32>(5).unwrap(),
                                    value: v.raw_get::<i32, f32>(6).unwrap(),
                                    step_by: v.raw_get::<i32, f64>(7).unwrap(),
                                    deicimals: v.raw_get::<i32, usize>(8).unwrap(),
                                    keybinding: None,
                                });
                                generator.lua.globals().set(k, control.clone()).unwrap();
                                new_controls.push(control);
                            }
                            "TextInput" => {
                                let control = Control::TextInputType(TextInput {
                                    name: v.raw_get::<i32, String>(2).unwrap(),
                                    label: v.raw_get::<i32, String>(3).unwrap(),
                                    value: v.raw_get::<i32, String>(4).unwrap(),
                                });
                                generator.lua.globals().set(k, control.clone()).unwrap();
                                new_controls.push(control);
                            }
                            "ComboBox" => {
                                let mut entries = vec![];
                                for i in 5..50 {
                                    match v.raw_get::<i32, String>(i) {
                                        Ok(s) => {
                                            entries.push(s);
                                        }
                                        Err(_) => {
                                            break;
                                        }
                                    }
                                }

                                let control = Control::ComboBoxType(ComboBoxInput {
                                    name: v.raw_get::<i32, String>(2).unwrap(),
                                    label: v.raw_get::<i32, String>(3).unwrap(),
                                    value: v.raw_get::<i32, String>(4).unwrap(),
                                    id: i,
                                    entries: entries,
                                });

                                generator.lua.globals().set(k, control.clone()).unwrap();
                                new_controls.push(control);
                            }
                            "Checkbox" => {
                                let control = Control::CheckboxType(CheckboxInput {
                                    name: v.raw_get::<i32, String>(2).unwrap(),
                                    label: v.raw_get::<i32, String>(3).unwrap(),
                                    value: v.raw_get::<i32, i32>(4).unwrap() == 1,
                                });
                                new_controls.push(control);
                            }
                            "Button" => {
                                let control = Control::ButtonType(ButtonInput {
                                    name: v.raw_get::<i32, String>(2).unwrap(),
                                    callback: v.raw_get::<i32, String>(3).unwrap(),
                                });
                                new_controls.push(control);
                            }
                            "Label" => {
                                let control = Control::Label(v.raw_get::<i32, String>(2).unwrap());
                                new_controls.push(control);
                            }
                            "Separator" => {
                                let control = Control::Separator;
                                new_controls.push(control);
                            }
                            "Spacer" => {
                                let control = Control::Spacer;
                                new_controls.push(control);
                            }
                            _ => {}
                        },
                        Err(_) => {}
                    }

                    Ok(())
                })
                .expect("Failed to parse table");

            generator.controls.append(&mut new_controls);
        }
    }

    pub fn tick_lua(&mut self) {
        for generator in &mut self.generators {
            generator.load();
            for control in &generator.controls {
                generator
                    .lua
                    .globals()
                    .set(control.get_name(), control.clone())
                    .unwrap();
            }
            let _ = generator
                .lua
                .globals()
                .get("tick")
                .and_then(|tick: Function<'_>| {
                    tick.call::<_, ()>(()).unwrap();
                    Ok(())
                });
        }
    }

    pub fn render_controls(&mut self, ui: &mut Ui) {
        let mut removed_generators = vec![];

        egui::Grid::new("Controls Grid").show(ui, |ui| {
            let mut last_generator_type = "".to_string();

            let mut i = 0;
            self.generators
                .iter_mut()
                .enumerate()
                .for_each(|(id, generator)| {
                    if last_generator_type == "" {
                        last_generator_type = generator.script.clone();
                    }

                    // new control types go to new rows
                    if &last_generator_type != &generator.script {
                        last_generator_type = generator.script.clone();
                        i = 0;
                        ui.end_row();
                    }
                    // otherwise two controls per row
                    else if i % 2 == 0 {
                        ui.end_row();
                    }

                    ui.vertical(|ui| {
                        for control in &mut generator.controls {
                            control.render(ui, &generator.lua);
                        }
                        if ui.button("Remove component").clicked() {
                            removed_generators.push(id);
                        }
                    });

                    i = i + 1;
                });
        });

        for generator in removed_generators {
            self.generators.remove(generator);
        }
    }

    pub fn generate_includes(&self) -> String {
        let mut code: String = "".to_string();

        for generator in &self.generators {
            let func: Result<Function<'_>, LuaError> =
                generator.lua.globals().get("generate_includes");

            if let Ok(f) = func {
                code += &*f.call::<_, String>(()).unwrap();
            }
        }
        code.to_string()
    }

    pub fn generate_init(&self) -> String {
        let mut code: String = "".to_string();

        for generator in &self.generators {
            let func: Result<Function<'_>, LuaError> = generator.lua.globals().get("generate_init");

            if let Ok(f) = func {
                code += &*f.call::<_, String>(()).unwrap();
            }
        }
        code.to_string()
    }

    pub fn generate_globals(&self) -> String {
        let mut code: String = "".to_string();

        for generator in &self.generators {
            let func: Result<Function<'_>, LuaError> =
                generator.lua.globals().get("generate_globals");

            if let Ok(f) = func {
                code += &*f.call::<_, String>(()).unwrap();
            }
        }
        code.to_string()
    }

    pub fn generate_loop_one_time_setup(&self) -> String {
        let mut code: String = "".to_string();

        for generator in &self.generators {
            let func: Result<Function<'_>, LuaError> =
                generator.lua.globals().get("generate_loop_one_time_setup");

            if let Ok(f) = func {
                code += &*f.call::<_, String>(()).unwrap();
            }
        }
        code.to_string()
    }

    pub fn generate_loop(&self) -> String {
        let mut code: String = "".to_string();

        for generator in &self.generators {
            let func: Result<Function<'_>, LuaError> = generator.lua.globals().get("generate_loop");

            if let Ok(f) = func {
                code += &*f.call::<_, String>(()).unwrap();
            }
        }
        code.to_string()
    }
}

impl UserData for LuaGenerator {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(_methods: &mut M) {
        /*methods.add_method_mut("add_slider", |_, this, (min, max, value): (f32, f32, f32)| {
            println!("Adding slider");
            this.controls.push(Control::SliderType(Slider {
                min: min,
                max: max,
                value: value,
                step_by: 1.0,
                deicimals: 0,
                label: "".to_string(),
                keybinding: None,
            }));
            Ok(())
        });*/
    }
}

impl LuaGenerator {
    pub fn new(script_path: &str) -> Self {
        let mut generator = Self {
            lua: Lua::new(),
            script: script_path.to_string(),
            script_data: "".into(),
            loaded: false,
            controls: vec![],
        };
        generator.render();
        generator
    }

    pub fn load(&mut self) {
        if let true = self.loaded {
            return;
        }

        self.loaded = true;

        let mut script_data: String = "".to_string();

        let file = File::open(self.script.clone());

        file.unwrap().read_to_string(&mut script_data).unwrap();

        self.script_data = script_data.to_string();

        self.lua.load(&self.script_data).exec().unwrap();
    }
    pub fn render(&mut self) {
        self.load();
        return;
    }
}

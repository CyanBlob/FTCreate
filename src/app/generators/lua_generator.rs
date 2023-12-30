use std::fs::File;
use std::io::Read;
use egui::Ui;
use mlua::{Function, Lua, Table, UserData, UserDataMethods};
use crate::app::generators::control::{Control};
use crate::app::generators::control::Control::SliderType;
use crate::app::generators::slider::{Slider};

pub struct ControlHandler {
    pub(crate) generators: Vec<LuaGenerator>,
}

pub struct LuaGenerator {
    pub lua: Lua,
    pub script: String,
    pub script_data: String,
    pub controls: Vec<Control>,
    pub loaded: bool,
}

impl ControlHandler {
    pub fn render(&mut self, ui: &mut Ui) {
        self.add_controls();

        self.render_controls(ui);

        self.tick_lua();
    }

    pub fn add_controls(&mut self)
    {
        for generator in &mut self.generators {
            let mut new_controls = vec![];

            let get_controls: Function<'_> = generator.lua.globals().get("get_controls").unwrap();
            let controls_changed: Function<'_> = generator.lua.globals().get("controls_changed").unwrap();

            let changed = controls_changed.call::<_, bool>(()).unwrap();

            if changed == false {
                break;
            }

            generator.controls.clear();

            let table = get_controls.call::<_, Table>(()).unwrap();

            println!("{:?}", table);

            table.for_each(|k: String, v: Table| {
                match v.raw_get::<i32, String>(1) {
                    Ok(s) => {
                        match s.as_str() {
                            "Slider" => {
                                let control = Control::SliderType(Slider {
                                    name: k.clone(),
                                    min: v.raw_get::<i32, f32>(2).unwrap(),
                                    max: v.raw_get::<i32, f32>(3).unwrap(),
                                    value: v.raw_get::<i32, f32>(4).unwrap(),
                                    step_by: v.raw_get::<i32, f64>(5).unwrap(),
                                    deicimals: v.raw_get::<i32, usize>(6).unwrap(),
                                    label: v.raw_get::<i32, String>(7).unwrap(),
                                    keybinding: None,
                                });
                                println!("Adding global with name: {}: {:?}", k, &control);
                                generator.lua.globals().set(k, control.clone()).unwrap();
                                new_controls.push(control);
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {}
                }

                Ok(())
            }).expect("Failed to parse table");


            generator.controls.append(&mut new_controls);
        }
    }

    pub fn tick_lua(&self)
    {
        for generator in &self.generators
        {
            for control in &generator.controls {
                generator.lua.globals().set(control.get_name(), control.clone()).unwrap();
            }
            let tick: Function<'_> = generator.lua.globals().get("tick").unwrap();
            tick.call::<_, ()>(()).unwrap();
        }
    }

    pub fn render_controls(&mut self, ui: &mut Ui)
    {
        for generator in &mut self.generators {
            for control in &mut generator.controls {
                control.render(ui);
            }
        }
    }
}

impl UserData for LuaGenerator {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
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
    pub fn render(&mut self)
    {
        self.load();
        return;
    }
}
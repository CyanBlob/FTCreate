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
        for generator in &mut self.generators {
            let mut new_controls = vec![];

            let p1: Function<'_> = generator.lua.globals().get("print1").unwrap();
            let p2: Function<'_> = generator.lua.globals().get("print2").unwrap();
            let get_controls: Function<'_> = generator.lua.globals().get("get_controls").unwrap();
            let controls_changed: Function<'_> = generator.lua.globals().get("controls_changed").unwrap();

            //TODO: temp
            generator.controls.clear();

            let table = get_controls.call::<_, Table>(()).unwrap();

            println!("{:?}", table);

            table.for_each(|k: String, v: Table| {
                match v.raw_get::<i32, String>(1) {
                    Ok(s) => {
                        match s.as_str() {
                            "Slider" => {
                                let control = Control::SliderType(Slider {
                                    min: v.raw_get::<i32, f32>(2).unwrap(),
                                    max: v.raw_get::<i32, f32>(3).unwrap(),
                                    value: v.raw_get::<i32, f32>(4).unwrap(),
                                    step_by: v.raw_get::<i32, f32>(5).unwrap(),
                                    deicimals: 0,
                                    label: "".to_string(),
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

            for control in &mut generator.controls {
                generator.lua.globals().set("myobject", control.clone()).unwrap();
                // TODO: Store the chunk for performance
                //generator.lua.load(&generator.script_data).exec().expect("Failed to run lua");
                //let str = self.lua.load(&self.script_data).eval::<String>().unwrap();


                let changed = controls_changed.call::<_, bool>(()).unwrap();

                /*if changed == false {
                    continue;
                }*/

                //let test_string = "Hey there!".to_string();
                //p1.call::<_, ()>(()).unwrap();
                //p2.call::<_, ()>(test_string).unwrap();

                control.render(ui);
            }
        }
    }
}

impl UserData for LuaGenerator {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("add_slider", |_, this, (min, max, value): (f32, f32, f32)| {
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
        });
    }
}

impl LuaGenerator {
    pub fn new(script_path: &str) -> Self {
        let mut generator = Self {
            lua: Lua::new(),
            script: script_path.to_string(),
            script_data: "".into(),
            loaded: false,
            controls: vec![
                Control::SliderType(Slider {
                    min: 0.0,
                    max: 100.0,
                    value: 10.0,
                    step_by: 0.5,
                    deicimals: 1,
                    label: "Test enum".to_string(),
                    keybinding: None,
                })
            ],
        };
        generator.render();
        //generator.lua.load(&generator.script_data).exec().expect("Failed to run lua");
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

        self.lua.scope(|scope| {
            // We create a 'sketchy' Lua callback that holds a mutable reference to the variable
            // `rust_val`. Outside of a `Lua::scope` call, this would not be allowed
            // because it could be unsafe.

            self.lua.globals().set(
                "test_add_slider",
                scope.create_function_mut(|_, (min, max, value, label): (f32, f32, f32, String)| {
                    println!("Called");
                    self.controls.push(
                        Control::SliderType(Slider {
                            min,
                            max,
                            value,
                            step_by: 1.0,
                            deicimals: 0,
                            label,
                            keybinding: None,
                        })
                    );
                    Ok(())
                })?,
            )?;

            //self.lua.load("test_add_slider(0, 25, 15, 'hmmm')").exec()

            Ok(())
        }).unwrap();
    }
    pub fn render(&mut self)
    {
        self.load();
        return;
    }
}
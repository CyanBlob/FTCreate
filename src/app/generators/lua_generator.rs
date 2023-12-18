use std::fs::File;
use std::io::Read;
use egui::Ui;
use mlua::{Lua};
use crate::app::generators::control::{Control};
use crate::app::generators::slider::{Slider};

pub struct LuaGenerator {
    pub lua: Lua,
    pub script: String,
    pub script_data: String,
    pub controls: Vec<Control>,
}

impl LuaGenerator {
    pub fn new(script_path: &str) -> Self {
        Self {
            lua: Lua::new(),
            script: script_path.to_string(),
            script_data: "".into(),
            controls: vec![
                Control::Slider(Slider {
                    min: 0.0,
                    max: 100.0,
                    value: 10.0,
                    step_by: 0.5,
                    deicimals: 1,
                    label: "Test enum".to_string(),
                    keybinding: None,
                })
            ],
        }
    }

    pub fn load(&mut self) {
        let mut script_data: String = "".to_string();

        let file = File::open(self.script.clone());

        file.unwrap().read_to_string(&mut script_data).unwrap();

        self.script_data = script_data.to_string();
    }
    pub fn render(&mut self, ui: &mut Ui)
    {
        self.load();
        for control in &mut self.controls {
            self.lua.globals().set("myobject", control.clone()).unwrap();
            // TODO: Store the chunk for performance
            self.lua.load(&self.script_data).exec().expect("Failed to run lua");
            let str = self.lua.load(&self.script_data).eval::<String>().unwrap();

            println!("{:?}", str);

            control.render(ui);
        }
    }
}
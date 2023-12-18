use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use egui::Ui;
use mlua::{Chunk, Function, Lua, UserData};
use crate::app::generators::control::{Control, ControlTest};
use crate::app::generators::slider::{Slider};

pub struct LuaGenerator<'lua> {
    pub lua: Lua,
    pub script: String,
    pub script_data: String,
    pub script_chunk: Option<Function<'lua>>,
    pub controls: Vec<ControlTest>,
}

impl LuaGenerator<'_> {

    pub fn new(script_path: &str) -> Self {
        let mut obj = Self {
            lua: Lua::new(),
            script: script_path.to_string(),
            script_data: "".into(),
            script_chunk: None,
            controls: vec![ ],
        };

        obj.controls.push(ControlTest::Slider(Slider{
            min: 0.0,
            max: 100.0,
            value: 10.0,
            step_by: 0.5,
            deicimals: 1,
            label: "Test enum".to_string(),
            keybinding: None,
        }));

        obj
    }

    pub fn load<'a>(&'a mut self) {
        let mut script_data: String  = "".to_string();

        let file = File::open(self.script.clone());

        file.unwrap().read_to_string(&mut script_data).unwrap();

        self.script_data = script_data.to_string();
        //let f = self.lua.load(script_data).into_function().unwrap();

        //self.script_chunk = Some(self.lua.load(script_data).into_function().unwrap());
    }
    pub fn render(&mut self, ui: &mut Ui)
    {
        self.load();
        for mut control in &mut self.controls {
            self.lua.globals().set("myobject", control.clone()).unwrap();
            self.lua.load(&self.script_data).exec();
            control.render(ui);
        }
    }
}
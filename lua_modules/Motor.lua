require "lua_private.Helpers"

controlsChanged = true

function get_controls()
    controls = {}

    controls[1] = pack("Label", "LabelTest")

    if LuaSlider2 ~= nil then
        controls[2] = pack("Slider", "MainSlider2", "Main Slider 2", 0, 30, LuaSlider2.value, 1, 2)
    else
        controls[2] = pack("Slider", "MainSlider2", "Main Slider 2", 0, 30, 0, 1, 2)
    end

    controls[3] = pack("Separator")

    controls[4] = pack("Spacer")

    controls[5] = pack("Label", "OtherControls")
    controls[6] = pack("TextInput", "PS2_Name", "PrintSlider2_Name", "PrintSlider2_Name")

    controls[7] = pack("ComboBox", "ComboTest", "Test combo", "One", "Two", "Three")

    controls[8] = pack("Checkbox", "CheckTest", "Test", 1)
    controls[9] = pack("Checkbox", "CheckTest2", "Test", 0)

    controlsChanged = false

    return controls
end

function tick()
    if PS2_Name ~= nil then
    end
end

function controls_changed()
    return controlsChanged
end

function generate_includes()
    return "// lua includes\n"
end

function generate_globals()
    return "// lua globals\n"
end

function generate_init()
    return "// lua init\n"
end

function generate_loop_one_time_setup()
    return "// lua loop one time setup\n"
end

function generate_loop()
    string = ""
    return string
end

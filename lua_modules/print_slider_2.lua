controlsChanged = true

function pack(...)
    return { n = select("#", ...), ... }
end

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

    controlsChanged = false

    return controls
end

function tick()
    if PS2_Name ~= nil then
        print("Has name")
        print(PS2_Name.text)
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
    string = string .. "// lua loop code!! Val: " .. tostring(MainSlider2.value) .. "\n"
    string = string .. "// lua loop code!! Val: " .. tostring(PS2_Name.text) .. "\n"
    string = string .. "// lua loop code!! Val: " .. tostring(ComboTest.text) .. "\n"
    return string
end

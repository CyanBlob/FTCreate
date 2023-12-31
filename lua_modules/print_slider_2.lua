controlsChanged = true

function pack(...)
    return {n = select("#", ...), ...}
end

function get_controls()
    controls = {}

    if LuaSlider2 ~= nil then
        controls["LuaSlider2"] = pack("Slider", 0, 30, LuaSlider2.value, 1, 2, "Main Slider 2")
    else
        controls["LuaSlider2"] = pack("Slider", 0, 30, 0, 1, 2, "Main Slider 2")
    end

    controls["PS2_Name"] = pack("TextInput", "PrintSlider2_Name", "PrintSlider2_Name")

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
    if LuaSlider2 ~= nil then
        return "// lua loop code!! Val: " .. tostring(LuaSlider2.value) .. "\n"
    end
    return "// lua loop code!!\n"
end
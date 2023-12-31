controlsChanged = true

function pack(...)
    return { n = select("#", ...), ... }
end

function get_controls()
    controls = {}

    --if LuaSlider ~= nil then

    --if LuaSlider.value == 20 then
    --print("Adding new slider")
    --controls["AnotherSlider"] = pack("Slider", "Another Slider", 0, 1, .5, .1, 2)
    --end

    --print("Val: ", LuaSlider.value)
    --controls["LuaSlider"] = pack("Slider", "Main Slider", 0, 30, LuaSlider.value, 1, 2)
    --else
    --controls["LuaSlider"] = pack("Slider", "Main Slider", 0, 30, 0, 1, 2)
    --end

    controlsChanged = false

    return controls
end

function tick()
    if LuaSlider ~= nil then
        print("Val: ", LuaSlider.value)
    end
end

function controls_changed()
    if LuaSlider ~= nil then
        return LuaSlider.value == 20
    end
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
    if LuaSlider ~= nil then
        return "// lua loop code!! Val: " .. tostring(LuaSlider.value) .. "\n"
    end
    return "// lua loop code!!\n"
end

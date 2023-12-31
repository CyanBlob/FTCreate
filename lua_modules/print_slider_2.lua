controlsChanged = true

function pack(...)
    return {n = select("#", ...), ...}
end

function get_controls()
    controls = {}

    if LuaSlider2 ~= nil then

        if LuaSlider2.value == 20 then
            print("Adding new slider")
            controls["AnotherSlider2"] = pack("Slider", 0, 1, .5, .1, 2, "Another Slider 2")
        end

        print("Val: 2 ", LuaSlider2.value)
        controls["LuaSlider2"] = pack("Slider", 0, 30, LuaSlider2.value, 1, 2, "Main Slider 2")
    else
        controls["LuaSlider2"] = pack("Slider", 0, 30, 0, 1, 2, "Main Slider 2")
    end

    controlsChanged = false

    return controls
end

function tick()
    if LuaSlider2 ~= nil then
        print("Val 2: ", LuaSlider2.value)
    end
end

function controls_changed()
    if LuaSlider2 ~= nil then
        print("Test")
        return LuaSlider2.value == 20
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
    if LuaSlider2 ~= nil then
        return "// lua loop code!! Val: " .. tostring(LuaSlider2.value) .. "\n"
    end
    return "// lua loop code!!\n"
end
controlsChanged = true

function pack(...)
    return {n = select("#", ...), ...}
end

function get_controls()
    controls = {}

    if LuaSlider ~= nil then

        if LuaSlider.value == 20 then
            print("Adding new slider")
            controls["AnotherSlider"] = pack("Slider", 0, 1, .5, .1, 2, "Another Slider")
        end

        print("Val: ", LuaSlider.value)
        controls["LuaSlider"] = pack("Slider", 0, 30, LuaSlider.value, 1, 2, "Main Slider")
    else
        controls["LuaSlider"] = pack("Slider", 0, 30, 0, 1, 2, "Main Slider")
    end

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
end

function generate_loop_one_time_setup()
end

function generate_loop()
    return "// lua loop code!"
end

function generate_globals()
end

function generate_init()
end

controlsChanged = true

function print1()
    print("From 1")
    print(myobject.value)
    --print(test_add_slider)
    --test_add_slider(0, 5, 2, "Test slider")
end

function print2(message)
    print("From 2")
    print(myobject.value)
    print(message)
end
function pack(...)
    return {n = select("#", ...), ...}
end

function get_controls()
    controls = {}
    if LuaSlider ~= nil then
        print("Val: ", LuaSlider.value)
        controls["LuaSlider"] = pack("Slider", 0, 30, LuaSlider.value, 1, 2)
    else
        controls["LuaSlider"] = pack("Slider", 0, 30, 0, 1, 2)
    end

    controlsChanged = false

    return controls
end

function controls_changed()
    return controlsChanged
end

function generate_includes()
end

function generate_loop_one_time_setup()
end

function generate_loop()
end

function generate_globals()
end

function generate_init()
end

function pack(...)
    return { n = select("#", ...), ... }
end

function exists(obj)
    return obj ~= nil
end

function inc(val)
    return val + 1
end

function checkbox(name, label, default)
    if exists(_G[name]) then
        return pack("Checkbox", name, label, _G[name].value)
    else
        return pack("Checkbox", name, label, default)
    end
end

function textInput(name, label, default)
    if exists(_G[name]) then
        return pack("TextInput", name, label, _G[name].text)
    else
        return pack("TextInput", name, label, default)
    end
end

function comboBox(name, label, default, ...)
    if exists(_G[name]) then
        return pack("ComboBox", name, label, _G[name].text, ...)
    else
        return pack("ComboBox", name, label, default, ...)
    end
end

function slider(name, label, min, max, default, step, decimals)
    if exists(_G[name]) then
        return pack("Slider", name, label, min, max,
            _G[name].value, step, decimals)
    else
        return pack("Slider", name, label, min, max,
            default, step, default)
    end
end

function label(text)
    return pack("Label", text)
end

function separator()
    return pack("Separator")
end

function spacer()
    return pack("Spacer")
end

keybindings = {}

keybindings["default_button"] = "button"
keybindings["a"] = "button"
keybindings["b"] = "button"
keybindings["x"] = "button"
keybindings["y"] = "button"
keybindings["left_stick_button"] = "button"
keybindings["right_stick_button"] = "button"
keybindings["left_bumper"] = "button"
keybindings["right_bumper"] = "button"
keybindings["dpad_left"] = "button"
keybindings["dpad_right"] = "button"
keybindings["dpad_up"] = "button"
keybindings["dpad_down"] = "button"
keybindings["start"] = "button"
keybindings["select"] = "button"

keybindings["default_axis"] = "axis"
keybindings["left_trigger"] = "axis"
keybindings["right_trigger"] = "axis"
keybindings["left_stick_x"] = "axis"
keybindings["left_stick_y"] = "axis"
keybindings["right_stick_x"] = "axis"
keybindings["right_stick_y"] = "axis"

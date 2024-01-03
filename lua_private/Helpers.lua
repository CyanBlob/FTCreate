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

function buttonComboBox(name, label, default)
    if exists(_G[name]) then
        return pack("ComboBox", name, label, _G[name].text,
            "default_button",
            "a",
            "b",
            "x",
            "y",
            "left_stick_button",
            "right_stick_button",
            "left_bumper",
            "right_bumper",
            "dpad_left",
            "dpad_right",
            "dpad_up",
            "dpad_down",
            "start",
            "select"
        )
    else
        return pack("ComboBox", name, label, default,
            "default_button",
            "a",
            "b",
            "x",
            "y",
            "left_stick_button",
            "right_stick_button",
            "left_bumper",
            "right_bumper",
            "dpad_left",
            "dpad_right",
            "dpad_up",
            "dpad_down",
            "start",
            "select"
        )
    end
end

function axisComboBox(name, label, default)
    if exists(_G[name]) then
        return pack("ComboBox", name, label, _G[name].text,
            "default_axis",
            "left_trigger",
            "right_trigger",
            "left_stick_x",
            "left_stick_y",
            "right_stick_x",
            "right_stick_y"
        )
    else
        return pack("ComboBox", name, label, default,
            "default_axis",
            "left_trigger",
            "right_trigger",
            "left_stick_x",
            "left_stick_y",
            "right_stick_x",
            "right_stick_y"
        )
    end
end


function keybindingComboBox(name, label, default)
    if exists(_G[name]) then
        return pack("ComboBox", name, label, _G[name].text,
            "default_button",
            "a",
            "b",
            "x",
            "y",
            "left_stick_button",
            "right_stick_button",
            "left_bumper",
            "right_bumper",
            "dpad_left",
            "dpad_right",
            "dpad_up",
            "dpad_down",
            "start",
            "select",
            "default_axis",
            "left_trigger",
            "right_trigger",
            "left_stick_x",
            "left_stick_y",
            "right_stick_x",
            "right_stick_y"
        )
    else
        return pack("ComboBox", name, label, default,
            "default_button",
            "a",
            "b",
            "x",
            "y",
            "left_stick_button",
            "right_stick_button",
            "left_bumper",
            "right_bumper",
            "dpad_left",
            "dpad_right",
            "dpad_up",
            "dpad_down",
            "start",
            "select",
            "default_axis",
            "left_trigger",
            "right_trigger",
            "left_stick_x",
            "left_stick_y",
            "right_stick_x",
            "right_stick_y"
        )
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

function isAxis(keybinding)
    return keybindings[keybinding] == "axis"
end

function isButton(keybinding)
    return keybindings[keybinding] == "button"
end


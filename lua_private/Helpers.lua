function pack(...)
    return { n = select("#", ...), ... }
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

require "lua_private.Helpers"

controlsChanged = true

run_mode = "Run using encoders"
num_positions = 0
is_drivetrain_servo = 0
drivetrain_type = ""

function get_controls()
    controls = {}

    index = 1

    controls[index] = label("Servo")
    index = index + 1

    controls[index] = spacer()
    index = index + 1

    controls[index] = textInput("SERVO_Name", "Servo", "Servo")
    index = index + 1

    -- fixed positons
    controls[index] = slider("SERVO_NumPositions", "Number of positions", 0, 10, 0, 1, 0)
    index = index + 1

    for i = 1, num_positions, 1 do
        controls[index] = spacer()
        index = index + 1

        controls[index] = slider("SERVO_Position" .. i, "Position: " .. i, -1, 1, 0, .01, 2)
        index = index + 1

        controls[index] = keybindingComboBox("SERVO_Keybind" .. i, "Keybinding", "none")
        index = index + 1

        controls[index] = spacer()
        index = index + 1
        controls[index] = separator()
        index = index + 1
    end
    controls[index] = spacer()
    index = index + 1


    controls[index] = comboBox("SERVO_Direction", "Direction", "Forward", "Forward", "Reverse")
    index = index + 1

    controlsChanged = false

    return controls
end

function tick()
    if exists(SERVO_NumPositions) then
        if SERVO_NumPositions.value ~= num_positions then
            num_positions = SERVO_NumPositions.value
            controlsChanged = true
        end
    end
end

function controls_changed()
    return controlsChanged
end

function generate_includes()
    return "import com.qualcomm.robotcore.hardware.CRServo;\n" ..
        "import org.firstinspires.ftc.robotcore.external.Telemetry;\n" ..
        "import com.qualcomm.robotcore.hardware.HardwareMap;\n"
end

function generate_globals()
    string = ""
    if exists(SERVO_Name) then
        string = string .. "private CRServo " .. SERVO_Name.text .. " = null;\n\n"
    end
    return string
end

function generate_init()
    string = ""

    if exists(SERVO_Name) then
        string = string .. '// ' .. SERVO_Name.text .. ' init\n' ..
            SERVO_Name.text .. ' = hardwareMap.get(CRServo.class, "' .. SERVO_Name.text .. '");\n'

        if SERVO_Direction.text == "Reverse" then
            string = string .. SERVO_Name.text .. '.setDirection(CRServo.Direction.REVERSE);\n'
        else
            string = string .. SERVO_Name.text .. '.setDirection(CRServo.Direction.FORWARD);\n'
        end
    end
    return string
end


function generate_loop()
    return generate_normal_loop()
end

function generate_normal_loop()
    string = ""
    default_string = ""
    added_first = false
    for i = 1, num_positions, 1 do
        position = _G["SERVO_Position" .. i]
        if exists(position) then
            keybind = _G["SERVO_Keybind" .. i]
            if isButton(keybind.text) then
                if keybind.text == "default_button" then -- This has to come last in the generated code, so save it and add it later
                    default_string = default_string .. "else {\n"
                    default_string = default_string ..
                    "\t" .. SERVO_Name.text .. ".setPower(" .. position.text .. ");\n"
                    default_string = default_string .. "}\n"
                else
                    if added_first == false then
                        string = string .. "if (gamepad1." .. keybind.text .. " > 0) {\n"
                        added_first = true
                    else
                        string = string .. "else if (gamepad1." .. keybind.text .. " > 0) {\n"
                    end
                    string = string .. "\t" .. SERVO_Name.text .. ".setPower(" .. position.value .. ");\n"
                    string = string .. "}\n"
                end
            end
            if isAxis(_G["SERVO_Keybind" .. i].text) then -- This is in "run to position" mode so we just set the target position if the axis is pushed at all
                -- TODO: Add a "scaled position" control where we set "Position * axis value)
                string = string .. "if (gamepad1." .. keybind.text .. " > 0) {\n"
                string = string .. "\t" .. SERVO_Name.text .. ".setPower(" .. position.value .. ");\n"
                string = string .. "}\n"
            end
        end
    end
    string = string .. default_string
    return string
end

require "lua_private.Helpers"

controlsChanged = true

run_mode = "Run using encoders"
num_positions = 0

function get_controls()
    controls = {}

    index = 1

    controls[index] = label("DC Motor")
    index = index + 1

    controls[index] = checkbox("DCM_IsDrivetrain", "Is Drivetrain Motor", 0)
    index = index + 1

    controls[index] = spacer()
    index = index + 1

    controls[index] = textInput("DCM_Name", "DC Motor", "DC_Motor")
    index = index + 1

    controls[index] = comboBox("DCM_RunMode", "Run Mode", "Run using encoders", "Run using encoders",
        "Run without encoders",
        "Run to position")
    index = index + 1

    -- fixed positons
    if run_mode == "Run to position" then
        controls[index] = slider("DCM_NumPositions", "Number of positions", 0, 10, 0, 1, 0)
        index = index + 1

        for i = 1, num_positions, 1 do
            controls[index] = spacer()
            index = index + 1

            controls[index] = slider("DCM_Position" .. i, "Position: " .. i, 0, 20000, 0, 1, 0)
            index = index + 1

            controls[index] = keybindingComboBox("DCM_Keybind" .. i, "Keybinding", "default_button")
            index = index + 1

            controls[index] = spacer()
            index = index + 1
            controls[index] = separator()
            index = index + 1
        end
        controls[index] = spacer()
        index = index + 1
    else -- normal run mode
        controls[index] = spacer()
        index = index + 1
        controls[index] = keybindingComboBox("DCM_Keybind", "Keybinding", "default_button")
        index = index + 1
        controls[index] = spacer()
        index = index + 1
        controls[index] = separator()
        index = index + 1
    end

    controls[index] = comboBox("DCM_Direction", "Direction", "Forward", "Forward", "Reverse")
    index = index + 1


    if run_mode == "Run to position" then
        controls[index] = slider("DCM_MaxSpeed", "Max Speed", 0, 12000, 0, .01, 2)
        index = index + 1
    else
        controls[index] = slider("DCM_MaxPower", "Max Power", 0, 1, 1, .01, 2)
        index = index + 1
    end

    controlsChanged = false

    return controls
end

function tick()
    if exists(DCM_RunMode) then
        if run_mode ~= DCM_RunMode.text then
            controlsChanged = true
            run_mode = DCM_RunMode.text
        end
    end
    if exists(DCM_NumPositions) then
        if DCM_NumPositions.value ~= num_positions then
            num_positions = DCM_NumPositions.value
            controlsChanged = true
        end
    end
end

function controls_changed()
    return controlsChanged
end

function generate_includes()
    return "import com.qualcomm.robotcore.hardware.DcMotor;\n" ..
        "import org.firstinspires.ftc.robotcore.external.Telemetry;\n" ..
        "import com.qualcomm.robotcore.hardware.HardwareMap;\n" ..
        "import com.qualcomm.robotcore.hardware.DcMotorEx;\n" ..
        "import com.qualcomm.robotcore.hardware.DcMotorSimple;\n\n"
end

function generate_globals()
    string = ""
    if exists(DCM_Name) then
        string = string .. "private DcMotorEx " .. DCM_Name.text .. " = null;\n\n"
    end
    return string
end

function generate_init()
    string = ""

    if exists(DCM_Name) and exists(DCM_RunMode) then
        string = string .. '// ' .. DCM_Name.text .. ' init\n' ..
            DCM_Name.text .. ' = hardwareMap.get(DcMotorEx.class, "' .. DCM_Name.text .. '");\n'

        if DCM_Direction.text == "Reverse" then
            string = string .. DCM_Name.text .. '.setDirection(DcMotor.Direction.REVERSE);\n'
        else
            string = string .. DCM_Name.text .. '.setDirection(DcMotor.Direction.FORWARD);\n'
        end

        string = string .. DCM_Name.text .. '.setMode(DcMotor.RunMode.STOP_AND_RESET_ENCODER);\n' ..
            DCM_Name.text .. '.setTargetPosition(0);\n'

        if DCM_RunMode.text == "Run to position" then
            string = string .. DCM_Name.text .. '.setMode(DcMotor.RunMode.RUN_TO_POSITION);\n'
        elseif DCM_RunMode.text == "Run using encoders" then
            string = string .. DCM_Name.text .. '.setMode(DcMotor.RunMode.RUN_USING_ENCODERS);\n'
        else
            string = string .. DCM_Name.text .. '.setMode(DcMotor.RunMode.RUN_WITHOUT_ENCODERS);\n'
        end
    end
    return string
end

function generate_loop_one_time_setup()
    return ""
end

function generate_loop()
    if not exists(DCM_IsDrivetrain) then
        return ""
    end
    if DCM_IsDrivetrain.value == 1 then
        return generate_drivetrain_loop()
    end

    return generate_normal_loop()
end

function generate_drivetrain_loop()
    return ""
end

function generate_normal_loop()
    string = ""
    if run_mode == "Run to position" then
        for i = 1, num_positions, 1 do
            position = _G["DCM_Position" .. i]
            if exists(position) then
                keybind = _G["DCM_Keybind" .. i]
                if isButton(keybind.text) then
                    string = string .. "if (gamepad1." .. keybind.text .. ") {\n"
                    string = string .. "\t" .. DCM_Name.text .. ".setTargetPosition(" .. position.value .. ");\n"
                    string = string .. "\t" .. DCM_Name.text .. ".setMode(DcMotor.RUN_TO_POSITION);\n"
                    string = string .. "\t" .. DCM_Name.text .. ".setVelocity(" .. DCM_MaxSpeed.value .. ");\n"
                    string = string .. "}\n"
                end
                if isAxis(_G["DCM_Keybind" .. i].text) then
                    string = string .. "if (gamepad1." .. keybind.text .. " > 0) {\n"
                    string = string .. "\t" .. DCM_Name.text .. ".setPower(gamepad1." .. position.value .. ");\n"
                    string = string .. "}\n"
                end
            end
        end
    else
        keybind = DCM_Keybind
        if isButton(keybind.text) then
            string = string .. "if (gamepad1." .. keybind.text .. ") {\n"
            string = string .. "\t" .. DCM_Name.text .. ".setPower(" .. DCM_MaxPower.value .. ");\n"
            string = string .. "}\n"
        else
            string = string .. DCM_Name.text .. ".setPower(gamepad1." .. keybind.text .. " * " .. DCM_MaxPower.value .. ");\n"
        end
    end
    return string
end

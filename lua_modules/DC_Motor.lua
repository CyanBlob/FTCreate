require "lua_private.Helpers"

controlsChanged = true

run_mode = "Run using encoders"
num_positions = 0
is_drivetrain_motor = 0
drivetrain_type = ""

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

    if is_drivetrain_motor == 1 then
        controls[index] = comboBox("DCM_RunMode", "Run Mode", "Run using encoders", "Run using encoders",
            "Run without encoders")
    else
        controls[index] = comboBox("DCM_RunMode", "Run Mode", "Run using encoders", "Run using encoders",
            "Run without encoders",
            "Run to position")
    end
    index = index + 1

    -- fixed positons
    if run_mode == "Run to position" then
        controls[index] = slider("DCM_NumPositions", "Number of positions", 0, 10, 0, 1, 0)
        index = index + 1

        for i = 1, num_positions, 1 do
            controls[index] = spacer()
            index = index + 1

            controls[index] = slider("DCM_Position" .. i, "Position: " .. i, -20000, 20000, 0, 1, 0)
            index = index + 1

            controls[index] = keybindingComboBox("DCM_Keybind" .. i, "Keybinding", "none")
            index = index + 1

            controls[index] = spacer()
            index = index + 1
            controls[index] = separator()
            index = index + 1
        end
        controls[index] = spacer()
        index = index + 1
    elseif is_drivetrain_motor == 0 then -- normal run mode (when not set as a drivetrain motor)
        controls[index] = spacer()
        index = index + 1
        controls[index] = keybindingComboBox("DCM_Keybind", "Keybinding", "none")
        index = index + 1
        controls[index] = spacer()
        index = index + 1
        controls[index] = separator()
        index = index + 1
    else -- drivetrain
        controls[index] = spacer()
        index = index + 1

        controls[index] = comboBox("DCM_DrivetrainType", "Drivetrain Type", "Mecanum", "Mecanum", "Arcade", "Tank")
        index = index + 1
    end

    if is_drivetrain_motor == 1 then
        if drivetrain_type == "Mecanum" then
            controls[index] = comboBox("DCM_MecanumPosition", "Mecanum Position", "Front Left", "Front Left",
                "Front Right", "Rear Left", "Rear Right")
            index = index + 1
        end
        if drivetrain_type == "Arcade" then
            controls[index] = comboBox("DCM_ArcadePosition", "Arcade Position", "Left",
                "Left", "Right")
            index = index + 1
        end
        if drivetrain_type == "Tank" then
            controls[index] = comboBox("DCM_TankPosition", "Tank Position", "Left",
                "Left", "Right")
            index = index + 1
        end

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
    if exists(DCM_IsDrivetrain) then
        if DCM_IsDrivetrain.value ~= is_drivetrain_motor then
            is_drivetrain_motor = DCM_IsDrivetrain.value
            controlsChanged = true

            if exists(DCM_RunMode) then
                if DCM_RunMode.text == "Run to position" then
                    DCM_RunMode.text = "Run using encoders"
                end
            end
        end
    end

    if exists(DCM_DrivetrainType) then
        if drivetrain_type ~= DCM_DrivetrainType.text then
            drivetrain_type = DCM_DrivetrainType.text
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
        string = string .. "\n"
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
    string = ""
    if exists(DCM_MaxPower) then
        power = DCM_MaxPower.text
    end
    if drivetrain_type == "Mecanum" then
        if exists(DCM_MecanumPosition) then
            if DCM_MecanumPosition.text == "Front Left" then
                string = string ..
                    DCM_Name.text .. ".setPower(Range.clip(drive - strafe + turn, -" .. power .. ", " .. power .. "));\n"
            elseif DCM_MecanumPosition.text == "Front Right" then
                string = string ..
                    DCM_Name.text .. ".setPower(Range.clip(drive + strafe - turn, -" .. power .. ", " .. power .. "));\n"
            elseif DCM_MecanumPosition.text == "Rear Left" then
                string = string ..
                    DCM_Name.text .. ".setPower(Range.clip(drive + strafe + turn, -" .. power .. ", " .. power .. "));\n"
            elseif DCM_MecanumPosition.text == "Rear Right" then
                string = string ..
                    DCM_Name.text .. ".setPower(Range.clip(drive - strafe - turn, -" .. power .. ", " .. power .. "));\n"
            end
        end
    elseif drivetrain_type == "Arcade" then
        if exists(DCM_ArcadePosition) then
            if DCM_ArcadePosition.text == "Left" then
                string = string ..
                    DCM_Name.text .. ".setPower(Range.clip(drive + turn, -" .. power .. ", " .. power .. "));\n"
            elseif DCM_ArcadePosition.text == "Right" then
                string = string ..
                    DCM_Name.text .. ".setPower(Range.clip(drive - turn, -" .. power .. ", " .. power .. "));\n"
            end
        end
    elseif drivetrain_type == "Tank" then
        if exists(DCM_TankPosition) then
            if DCM_TankPosition.text == "Left" then
                string = string ..
                    DCM_Name.text .. ".setPower(Range.clip(driveLeft, -" .. power .. ", " .. power .. "));\n"
            elseif DCM_TankPosition.text == "Right" then
                string = string ..
                    DCM_Name.text .. ".setPower(Range.clip(driveRight, -" .. power .. ", " .. power .. "));\n"
            end
        end
    end
    return string
end

function generate_normal_loop()
    if is_drivetrain_motor == 1 then
        return ""
    end
    string = ""
    default_string = ""
    added_first = false
    if run_mode == "Run to position" then
        for i = 1, num_positions, 1 do
            position = _G["DCM_Position" .. i]
            if exists(position) then
                keybind = _G["DCM_Keybind" .. i]
                if isButton(keybind.text) then
                    if keybind.text == "default_button" then -- This has to come last in the generated code, so save it and add it later
                        default_string = default_string .. "// Default case; what happens when no button is pressed\n"
                        default_string = default_string .. "else {\n"
                        default_string = default_string ..
                            "\t" .. DCM_Name.text .. ".setTargetPosition(" .. position.text .. ");\n"
                        default_string = default_string ..
                            "\t" .. DCM_Name.text .. ".setMode(DcMotor.RUN_TO_POSITION);\n"
                        default_string = default_string ..
                            "\t" .. DCM_Name.text .. ".setVelocity(" .. DCM_MaxSpeed.text .. ");\n"
                        default_string = default_string .. "}\n"
                    else
                        if added_first == false then
                            string = string .. "if (gamepad1." .. keybind.text .. ") {\n"
                            added_first = true
                        else
                            string = string .. "else if (gamepad1." .. keybind.text .. ") {\n"
                        end
                        string = string .. "\t" .. DCM_Name.text .. ".setTargetPosition(" .. position.text .. ");\n"
                        string = string .. "\t" .. DCM_Name.text .. ".setMode(DcMotor.RUN_TO_POSITION);\n"
                        string = string .. "\t" .. DCM_Name.text .. ".setVelocity(" .. DCM_MaxSpeed.text .. ");\n"
                        string = string .. "}\n"
                    end
                end
                if isAxis(_G["DCM_Keybind" .. i].text) then -- This is in "run to position" mode so we just set the target position if the axis is pushed at all
                    -- TODO: Add a "scaled position" control where we set "Position * axis value)
                    string = string .. "if (gamepad1." .. keybind.text .. " > 0) {\n"
                    string = string .. "\t" .. DCM_Name.text .. ".setPower(" .. position.value .. ");\n"
                    string = string .. "}\n"
                end
            end
        end
        string = string .. default_string
    else -- Run with/without encoders
        keybind = DCM_Keybind

        if isButton(keybind.text) then
            string = string .. "if (gamepad1." .. keybind.text .. ") {\n"
            string = string .. "\t" .. DCM_Name.text .. ".setPower(" .. DCM_MaxPower.text .. ");\n"
            string = string .. "}\n"
        elseif isAxis(keybind.text) then
            string = string ..
                DCM_Name.text .. ".setPower(gamepad1." .. keybind.text .. " * " .. DCM_MaxPower.text .. ");\n"
        end
    end
    return string
end

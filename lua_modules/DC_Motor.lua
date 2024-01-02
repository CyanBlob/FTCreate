require "lua_private.Helpers"

controlsChanged = true

run_mode = "Run using encoders"
num_positions = 0

function get_controls()
    controls = {}

    index = 1

    controls[index] = pack("Label", "DC Motor")
    index = index + 1

    if DCM_IsDrivetrain ~= nil then
        controls[index] = pack("Checkbox", "DCM_IsDrivetrain", "Is Drivetrain Motor", DCM_IsDrivetrain.value)
        index = index + 1
    else
        controls[index] = pack("Checkbox", "DCM_IsDrivetrain", "Is Drivetrain Motor", 0)
        index = index + 1
    end

    controls[index] = pack("Spacer")
    index = index + 1

    if DCM_Name ~= nil then
        controls[index] = pack("TextInput", "DCM_Name", "DC Motor", DCM_Name.text)
        index = index + 1
    else
        controls[index] = pack("TextInput", "DCM_Name", "DC Motor", "DC_Motor")
        index = index + 1
    end

    if DCM_RunMode ~= nil then
        controls[index] = pack("ComboBox", "DCM_RunMode", "Run Mode", DCM_RunMode.text, "Run using encoders",
            "Run without encoders",
            "Run to position")
        index = index + 1
    else
        controls[index] = pack("ComboBox", "DCM_RunMode", "Run Mode", "Run using encoders", "Run using encoders",
            "Run without encoders",
            "Run to position")
        index = index + 1
    end

    -- fixed positons
    if run_mode == "Run to position" then
        if DCM_NumPositions ~= nil then
            controls[index] = pack("Slider", "DCM_NumPositions", "Number of positions", 0, 10, DCM_NumPositions.value, 1,
                0)
            index = index + 1
        else
            controls[index] = pack("Slider", "DCM_NumPositions", "Number of positions", 0, 10, 0, 1, 0)
            index = index + 1
        end

        for i = 1, num_positions, 1 do
            -- Lookup slider in global table (_G)
            if _G["DCM_Position" .. i] ~= nil then
                controls[index] = pack("Slider", "DCM_Position" .. i, "Position: " .. i, 0, 20000,
                    _G["DCM_Position" .. i].value, 1, 0)
                index = index + 1

                -- Lookup keybinding
                controls[index] = pack("ComboBox", "DCM_Keybind" .. i, "Keybinding", _G["DCM_Keybind" .. i].text,
                    "default_button",
                    "a",
                    "b",
                    "x",
                    "y")
                index = index + 1
            else
                controls[index] = pack("Slider", "DCM_Position" .. i, "Position: " .. i, 0, 20000, 0, 1, 0)
                index = index + 1

                controls[index] = pack("ComboBox", "DCM_Keybind" .. i, "Keybinding", "default_button",
                    "default_button",
                    "a",
                    "b",
                    "x",
                    "y")
                index = index + 1
            end

            controls[index] = pack("Separator")
            index = index + 1
            controls[index] = pack("Spacer")
            index = index + 1
        end
    end

    if DCM_Direction ~= nil then
        controls[index] = pack("ComboBox", "DCM_Direction", "Direction", DCM_Direction.text, "Forward", "Reverse")
        index = index + 1
    else
        controls[index] = pack("ComboBox", "DCM_Direction", "Direction", "Forward", "Forward", "Reverse")
        index = index + 1
    end

    if run_mode == "Run to position" then
        controls[index] = pack("Slider", "DCM_MaxSpeed", "Max Speed", 0, 12000, 0, .01, 2)
        index = index + 1
    else
        controls[index] = pack("Slider", "DCM_MaxPower", "Max Power", 0, 1, 1, .01, 2)
        index = index + 1
    end

    controlsChanged = false

    return controls
end

function tick()
    if DCM_RunMode ~= nil then
        if run_mode ~= DCM_RunMode.text then
            controlsChanged = true
            run_mode = DCM_RunMode.text
        end
    end
    if DCM_NumPositions ~= nil then
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
    if DCM_Name ~= nil then
        string = string .. "private DcMotorEx " .. DCM_Name.text .. " = null;\n\n"
    end
    return string
end

function generate_init()
    string = ""

    if DCM_Name ~= nil then
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
    string = ""
    return string
end

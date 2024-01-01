require "lua_private.Helpers"

controlsChanged = true

run_mode = "Run using encoders"

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
    string = ""
    --string = string .. "// lua loop code!! Val: " .. tostring(MainSlider2.value) .. "\n"
    --string = string .. "// lua loop code!! Val: " .. tostring(PS2_Name.text) .. "\n"
    --string = string .. "// lua loop code!! Val: " .. tostring(ComboTest3.text) .. "\n"
    return string
end

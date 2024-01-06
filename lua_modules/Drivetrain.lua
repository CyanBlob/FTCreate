require "lua_private.Helpers"

controlsChanged = true

run_mode = "Run using encoders"
num_positions = 0

function get_controls()
    controls = {}

    index = 1

    controls[index] = label("Drivetrain config (needs motors)")
    index = index + 1

    controls[index] = spacer()
    index = index + 1

    controls[index] = comboBox("DCM_DrivetrainType", "Drivetrain Type", "Mecanum", "Mecanum",
        "Arcade",
        "Tank")
    index = index + 1

    controlsChanged = false

    return controls
end

function tick()
end

function controls_changed()
    return controlsChanged
end

function generate_loop_one_time_setup()
    string = ""
    if exists(DCM_DrivetrainType) then
        if DCM_DrivetrainType.text == "Mecanum" then
            string = string .. "\n// Mecanum drivetrain one time setup\n"
            string = string .. "Double drive  = gamepad1.left_stick_y;  // forwards and backwards movement\n"
            string = string .. "Double turn   = gamepad1.right_stick_x; // rotation\n"
            string = string .. "Double strafe = gamepad1.left_stick_x;  // side to side movement\n"
        elseif DCM_DrivetrainType.text == "Arcade" then
            string = string .. "\n// Arcade drivetrain one time setup\n"
            string = string .. "Double drive  = gamepad1.left_stick_y;  // forwards and backwards movement\n"
            string = string .. "Double turn   = gamepad1.right_stick_x; // rotation \n"
        elseif DCM_DrivetrainType.text == "Tank" then
            string = string .. "\n// Tank drivetrain one time setup\n"
            string = string .. "double driveLeft  = gamepad1.left_stick_y;  // left motors movement\n"
            string = string .. "double driveRight = gamepad1.right_stick_y; // right motors movement \n"
        end
    end
    return string
end

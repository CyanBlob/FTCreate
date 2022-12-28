use crate::generators::generator::Generator;
pub mod generators;

use generators::drivetrain;

use crate::drivetrain::drivetrain::Drivetrain;
use generators::motors::core_hd::CoreHD;
/*
                drivetrain: Drivetrain {
                    motors: vec![Box::new(CoreHD {
                        direction: generators::motors::motor::MotorDirection::FORWARD,
                        max_speed: 100.0,
                        position: 0,
                    })],
                },
*/
use druid::widget::{Align, Flex, Label, Padding};
use druid::{AppLauncher, Data, PlatformError, Widget, WindowDesc};

use std::rc::Rc;

#[derive(Clone, Data)]
struct State {
    drivetrain: Rc<Drivetrain<CoreHD>>,
}

fn build_ui(state: &State) -> impl Widget<()> {
    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new(state.drivetrain.motors.iter().nth(0).unwrap().generate().unwrap().as_str()), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom left")), 1.0),
                1.0,
            )
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top right"), 1.0)
                    .with_flex_child(Align::left(Label::new("bottom right")), 1.0),
                1.0,
            ),
    )
}

fn main() -> Result<(), PlatformError> {
    let state = State {
        drivetrain: Rc::new(Drivetrain {
            motors: vec![Box::new(CoreHD {
                direction: generators::motors::motor::MotorDirection::FORWARD,
                max_speed: 100.0,
                position: 0,
            })],
        }),
    };
    AppLauncher::with_window(WindowDesc::new(build_ui(&state))).launch(())?;
    Ok(())
}

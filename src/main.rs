use crate::generators::generator::Generator;
pub mod generators;

use generators::drivetrain;

use crate::drivetrain::drivetrain::Drivetrain;
use druid::widget::{Align, Button, Flex, Label, Padding};
use druid::{AppLauncher, Data, PlatformError, Widget, WindowDesc};
use generators::motors::core_hd::CoreHD;

#[derive(Clone, Debug, Data)]
struct AppState {
    drivetrain: Drivetrain<CoreHD>,
}

fn build_ui() -> impl Widget<AppState> {
    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(
                        Label::dynamic(|data: &AppState, _| {
                            data.drivetrain
                                .motors
                                .iter()
                                .nth(0)
                                .unwrap()
                                .generate()
                                .unwrap()
                                .as_str()
                                .to_string()
                        }),
                        1.0,
                    )
                    .with_flex_child(Align::centered(Label::new("bottom left")), 1.0),
                1.0,
            )
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top right"), 1.0)
                    .with_flex_child(Align::left(Label::new("bottom right")), 1.0),
                1.0,
            )
            .with_flex_child(
                Flex::column()
                    .with_flex_child(
                        Button::new("Increase speed").on_click(
                            |_event, data: &mut AppState, _env| {
                                let _data = data.clone();

                                data.drivetrain.motors.iter_mut().nth(0).unwrap().max_speed += 1.0;

                                println!("{:?}", data);
                                println!("{:?}", data.same(&_data));
                            },
                        ),
                        1.0,
                    )
                    .with_flex_child(
                        Button::new("Decrease speed").on_click(
                            |_event, data: &mut AppState, _env| {
                                let _data = data.clone();

                                data.drivetrain.motors.iter_mut().nth(0).unwrap().max_speed -= 1.0;

                                println!("{:?}", data);
                                println!("{:?}", data.same(&_data));
                            },
                        ),
                        1.0,
                    ),
                1.0,
            ),
    )
}

fn main() -> Result<(), PlatformError> {
    let state = AppState {
        drivetrain: //Rc::new(
            Drivetrain {
                motors: //Rc::new(
                    druid::im::vector![Box::new(CoreHD {
                        direction: generators::motors::motor::MotorDirection::FORWARD,
                        max_speed: 100.0,
                        position: 0,
                    })],
                //),
            },
    };
    AppLauncher::with_window(WindowDesc::new(build_ui()))
        .log_to_console()
        .launch(state)?;
    Ok(())
}

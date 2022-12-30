use crate::generators::generator::Generator;
pub mod generators;

use generators::drivetrain;
use generators::motors::motor::MotorGenerator;

use crate::drivetrain::drivetrain::Drivetrain;
use druid::widget::{Button, Flex, Label, Padding, List, ViewSwitcher, Container};
use druid::{AppLauncher, Data, PlatformError, Widget, WindowDesc, Lens, WidgetExt, LensExt};
use generators::motors::core_hd::CoreHD;

#[derive(Clone, Debug, Data, Lens)]
struct AppState {
    drivetrain: Drivetrain<CoreHD>,
    drivetrain_speed: f64
}


fn build_ui() -> impl Widget<AppState> {
    Padding::new(
        00.0,
        Flex::column()
            .with_flex_child(Label::new("Motors"), 1.0)
            .with_flex_child(render_generator::<CoreHD>(), 1.0)
            .with_flex_child(Button::new("Update").on_click(|_, data, _| {
                println!("{:?}", data);
            }), 1.0)
    )
}

fn render_generator<T: MotorGenerator>() -> impl Widget<AppState> {
    Flex::column().with_flex_child(
        List::new(|| {
            Container::new(
                ViewSwitcher::new(|_data: &CoreHD, _| {
                    0
                }
                    , |_selector, data, _env| {
                        println!("RENDERING {:?}", data);
                        Box::new(data.render_options())
                    }).fix_size(500.0, 125.0)
                )
        }).lens(AppState::drivetrain.then(Drivetrain::motors)) , 1.0)
}

/*fn render_motors() -> impl Widget<AppState> {
            Container::new(
                ViewSwitcher::new(|_data: &Drivetrain<CoreHD>, _| {
                    0
                }
                    , |_selector, data, _env| {
                        println!("RENDERING {:?}", data);

                let mut column = Flex::column();

                data.motors.iter().for_each(|motor| {
                    column.add_child(
                    //Label::new("TEST")
                    motor.render_options()
                    )
                });

                Box::new(data.render())
                //Box::new(Label::new("TEST"))
                        //data.motors.iter().nth(0).unwrap().render_options()
                
                    }).fix_size(500.0, 50.0)
                ).border(Color::AQUA, 1.0)
}*/

fn main() -> Result<(), PlatformError> {
    let state = AppState {
        drivetrain: //Rc::new(
        Drivetrain {
            motors: //Rc::new(
            druid::im::vector![CoreHD {
                direction: generators::motors::motor::MotorDirection::FORWARD,
                max_speed: 0.75,
                position: 0,
            },
                CoreHD {
                    direction: generators::motors::motor::MotorDirection::REVERSE,
                    max_speed: -1.0,
                    position: 0,
                }
            ],
            //),
        },
        drivetrain_speed: 0.0,
    };
    AppLauncher::with_window(WindowDesc::new(build_ui()))
        //.log_to_console()
        .launch(state)?;
    Ok(())
}

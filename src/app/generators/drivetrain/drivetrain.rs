use crate::app::generators::motors::motor::MotorGenerator;

#[cfg(allow_unused)]
enum DrivetrainType {
    MECANUM,
    TANK,
    SWERVE,
    ARCADE,
}

// TODO: vecs should be wrapped in Rc (or use Im vecs?)
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Drivetrain<T: MotorGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone> {
    pub motors: Vec<T>,
}

//impl<T: MotorGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone + druid::Data> Drivetrain<T> {
/*impl Drivetrain<CoreHD> {
    pub fn render(&self) -> Box::<dyn Widget<AppState>> {

        let mut column = Flex::column();

        self.motors.iter().for_each(|motor| {
            column.add_child(motor.render_options());
        });

        let container = Container::new(column);
        Box::new(container);

        self.motors.iter().nth(0).unwrap().render_options()
    }
}*/

/*impl<T: MotorGenerator + std::cmp::PartialEq + std::cmp::PartialOrd + std::clone::Clone> Widget<AppState> for Drivetrain<T> {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut AppState, env: &druid::Env) {
        
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &AppState, data: &AppState, env: &druid::Env) {
       ctx.request_paint();
    }

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &AppState, env: &druid::Env) -> druid::Size {
        druid::Size { width: 200.0, height: 200.0 }
        
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, env: &druid::Env) {
       ctx. 
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &AppState, env: &druid::Env) {
        
    }
}*/

use crate::model::Car;

pub trait Thinker {
    fn think(&mut self);
}

impl Thinker for Car {
    fn think(&mut self) {
        self.desired_speed = 130.;
        self.desired_steer = 0.01;
    }
}

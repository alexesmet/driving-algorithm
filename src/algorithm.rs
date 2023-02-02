use crate::model::Car;

pub trait Thinker {
    fn think(&mut self);
}

impl Thinker for Car {
    fn think(&mut self) {
        self.speed = 80.;
        self.steer = 0.01;
    }
}

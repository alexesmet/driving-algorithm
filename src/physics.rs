use crate::model::Car;

const UPS: i32 = 60;

pub trait Physics {
    fn update(&mut self);
}

impl Physics for Car {
    fn update(&mut self) {
        self.orientation += self.steer * self.speed / UPS as f32;

        self.coordinates.0 += self.speed * self.orientation.cos() / UPS as f32;
        self.coordinates.1 += self.speed * self.orientation.sin() / UPS as f32;
    }
}

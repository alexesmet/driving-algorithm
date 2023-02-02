use crate::model::Car;
use nannou::prelude::*;

pub trait Drawing {
    fn draw(&self, draw: &Draw);
}

impl Drawing for Car {
    fn draw(&self, draw: &Draw) {
        draw.rect()
            .width(40.0)
            .height(20.0)
            .rotate(self.orientation)
            .x_y(self.coordinates.0, self.coordinates.1)
            .color(STEELBLUE);
    }
}

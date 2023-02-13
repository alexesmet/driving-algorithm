use crate::model::Car;
use nannou::prelude::*;

pub trait Drawing {
    fn draw(&self, draw: &Draw);
}

const STEER_FACTOR: f32 = 40.0;

impl Drawing for Car {
    fn draw(&self, draw: &Draw) {

        let wheel_fl = get_coords_for_wheels(self.coordinates, self.orientation, ( 10., 9.));
        let wheel_fr = get_coords_for_wheels(self.coordinates, self.orientation, ( 10.,-9.));
        let wheel_bl = get_coords_for_wheels(self.coordinates, self.orientation, (-10., 9.));
        let wheel_br = get_coords_for_wheels(self.coordinates, self.orientation, (-10.,-9.));

        draw.rect()
            .width(7.0)
            .height(4.0)
            .x_y(wheel_fl.0, wheel_fl.1)
            .rotate(self.orientation + self.steer * STEER_FACTOR)
            .color(DARKSLATEGREY);

        draw.rect()
            .width(7.0)
            .height(4.0)
            .x_y(wheel_fr.0, wheel_fr.1)
            .rotate(self.orientation + self.steer * STEER_FACTOR)
            .color(DARKSLATEGREY);

        draw.rect()
            .width(7.0)
            .height(4.0)
            .x_y(wheel_bl.0, wheel_bl.1)
            .rotate(self.orientation)
            .color(DARKSLATEGREY);

        draw.rect()
            .width(7.0)
            .height(4.0)
            .x_y(wheel_br.0, wheel_br.1)
            .rotate(self.orientation)
            .color(DARKSLATEGREY);

        draw.rect()
            .width(40.0)
            .height(20.0)
            .rotate(self.orientation)
            .x_y(self.coordinates.0, self.coordinates.1)
            .color(STEELBLUE);

    }
}

fn get_coords_for_wheels(origin: (f32, f32), rot: f32, shift: (f32, f32)) -> (f32, f32) {
    let x = origin.0 + shift.0 * rot.cos() + shift.1 * -rot.sin();
    let y = origin.1 + shift.0 * rot.sin() + shift.1 *  rot.cos();
    return (x,y);
}

use crate::model::{Car, Road, Roundabout};
use nannou::prelude::*;

pub trait Drawing {
    fn draw(&self, draw: &Draw);
}

pub trait DrawingDebug {
    fn draw_debug(&self, draw: &Draw);
}


const STEER_FACTOR: f32 = 40.0;

impl Drawing for Car {
    fn draw(&self, draw: &Draw) {

        let wheel_fl = get_coords_for_wheels(self.position.coordinates, self.position.orientation, ( 10., 9.));
        let wheel_fr = get_coords_for_wheels(self.position.coordinates, self.position.orientation, ( 10.,-9.));
        let wheel_bl = get_coords_for_wheels(self.position.coordinates, self.position.orientation, (-10., 9.));
        let wheel_br = get_coords_for_wheels(self.position.coordinates, self.position.orientation, (-10.,-9.));

        draw.rect()
            .width(7.0)
            .height(4.0)
            .x_y(wheel_fl.0, wheel_fl.1)
            .rotate(self.position.orientation + self.steer * STEER_FACTOR)
            .color(DARKSLATEGREY);

        draw.rect()
            .width(7.0)
            .height(4.0)
            .x_y(wheel_fr.0, wheel_fr.1)
            .rotate(self.position.orientation + self.steer * STEER_FACTOR)
            .color(DARKSLATEGREY);

        draw.rect()
            .width(7.0)
            .height(4.0)
            .x_y(wheel_bl.0, wheel_bl.1)
            .rotate(self.position.orientation)
            .color(DARKSLATEGREY);

        draw.rect()
            .width(7.0)
            .height(4.0)
            .x_y(wheel_br.0, wheel_br.1)
            .rotate(self.position.orientation)
            .color(DARKSLATEGREY);

        draw.rect()
            .width(40.0)
            .height(20.0)
            .rotate(self.position.orientation)
            .x_y(self.position.coordinates.0, self.position.coordinates.1)
            .color(STEELBLUE);

    }
}

impl DrawingDebug for Car {
    fn draw_debug(&self, draw: &Draw) {
        if let Some(desired) = &self.debug.desired_position {
            draw.line()
                .start(pt2(self.position.coordinates.0, self.position.coordinates.1))
                .end(pt2(desired.coordinates.0, desired.coordinates.1))
                .weight(2.0)
                .color(GAINSBORO);

            draw.rect()
                .width(40.0)
                .height(20.0)
                .roll(desired.orientation)
                .x_y(desired.coordinates.0, desired.coordinates.1)
                .color(GAINSBORO);
        }

        if let Some(angle) = &self.debug.angle_to_position {
            let arrow_len = 30.0;
            draw.arrow()
                .start_cap_round()
                .head_length(8.0)
                .head_width(4.0)
                .color(DARKRED)
                .start(pt2(self.position.coordinates.0, self.position.coordinates.1))
                .end(pt2(self.position.coordinates.0 + (self.position.orientation + angle).cos() * arrow_len, 
                         self.position.coordinates.1 + (self.position.orientation + angle).sin() * arrow_len));

        }

        if let Some(angle) = &self.debug.angle_to_orientation {
            let arrow_len = 30.0;
            draw.arrow()
                .start_cap_round()
                .head_length(8.0)
                .head_width(4.0)
                .color(DARKGREEN)
                .start(pt2(self.position.coordinates.0, self.position.coordinates.1))
                .end(pt2(self.position.coordinates.0 + (self.position.orientation + angle).cos() * arrow_len, 
                         self.position.coordinates.1 + (self.position.orientation + angle).sin() * arrow_len));

        }

        if let Some(angle) = &self.debug.turning_angle {
            let arrow_len = 30.0;
            draw.arrow()
                .start_cap_round()
                .head_length(8.0)
                .head_width(4.0)
                .color(BLACK)
                .start(pt2(self.position.coordinates.0, self.position.coordinates.1))
                .end(pt2(self.position.coordinates.0 + (self.position.orientation + angle).cos() * arrow_len, 
                         self.position.coordinates.1 + (self.position.orientation + angle).sin() * arrow_len));

        }
    }
}



fn get_coords_for_wheels(origin: (f32, f32), rot: f32, shift: (f32, f32)) -> (f32, f32) {
    let x = origin.0 + shift.0 * rot.cos() + shift.1 * -rot.sin();
    let y = origin.1 + shift.0 * rot.sin() + shift.1 *  rot.cos();
    return (x,y);
}

impl Drawing for Roundabout {
    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .w(self.radius * 2.)
            .h(self.radius * 2.)
            .x_y(self.coordinates.0, self.coordinates.1)
            .stroke_weight(2.0)
            .stroke_color(GAINSBORO)
            .no_fill();

    }
}

impl Drawing for Road {
    fn draw(&self, draw: &Draw) {
        match self {
            Road::Turn { coordinates, radius, start_angle, end_angle } => {
                let total_angle = end_angle - start_angle;
                let angle_step = 5.0 * PI / radius;
                let steps = (total_angle / angle_step).ceil() as i32;
                let points = (0..=steps).map(|i| {
                    let angle = start_angle + i as f32 * angle_step;
                    pt2(coordinates.0 + radius * angle.cos(), coordinates.1 + radius * angle.sin())
                });

                draw.polyline()
                    .weight(2.0)
                    .color(GAINSBORO)
                    .points(points);

            },
        }
    }
}

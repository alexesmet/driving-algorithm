use std::f32::consts::PI;

use crate::model::Car;

const UPS: f32 = 60.;

pub trait Physics {
    fn update(&mut self);
}

const EPSILON: f32 = 1.0;
pub const BRAKES: f32 = 130.0 / UPS;
pub const ACCELLERATION: f32 = 40.0 / UPS;
pub const STEER_SPEED: f32 = 0.05 / UPS; // 0.01 is too hardcore for beginning, but is kinda real
pub const MAX_STEER: f32 = 0.02;
pub const MAX_SPEED: f32 = 170.0;

impl Physics for Car {
    fn update(&mut self) {

        if self.brakes && self.speed.abs() > EPSILON {
            self.speed =- BRAKES.min(self.speed.abs()) * self.speed.signum();
        } else {
            let speed_diff = self.desired_speed - self.speed;
            self.speed += ACCELLERATION.min(speed_diff.abs()) * speed_diff.signum();
            self.speed = MAX_SPEED.min(self.speed.abs()) * self.speed.signum();
        }

        let steer_diff = self.desired_steer - self.steer;
        self.steer += STEER_SPEED.min(steer_diff.abs()) * steer_diff.signum();
        self.steer = MAX_STEER.min(self.steer.abs()) * self.steer.signum();

        self.position.orientation += self.steer * self.speed / UPS;
        self.position.orientation = self.position.orientation - 2.0*PI * ((self.position.orientation + PI) / (2.0*PI)).floor();

        self.position.coordinates.0 += self.speed * self.position.orientation.cos() / UPS;
        self.position.coordinates.1 += self.speed * self.position.orientation.sin() / UPS;
    }
}

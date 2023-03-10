use crate::{model::Car, util::normalize_angle};

const UPS: f32 = 60.;

pub trait Physics {
    fn update(&mut self);
}

const EPSILON: f32 = 1.0;
pub const BRAKES: f32 = 130.0 / UPS;
pub const ACCELLERATION: f32 = 40.0 / UPS;
pub const STEER_SPEED: f32 = 0.025; // 0.03 for low speed // 0.01 for high speed
pub const MAX_STEER: f32 = 0.02;
pub const MAX_SPEED: f32 = 200.0;

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
        self.steer += (STEER_SPEED / UPS).min(steer_diff.abs()) * steer_diff.signum();
        self.steer = MAX_STEER.min(self.steer.abs()) * self.steer.signum();

        self.position.orientation += self.steer * self.speed / UPS;
        self.position.orientation = normalize_angle(self.position.orientation);

        self.position.coordinates.0 += self.speed * self.position.orientation.cos() / UPS;
        self.position.coordinates.1 += self.speed * self.position.orientation.sin() / UPS;
    }
}

use std::f32::consts::{FRAC_PI_4, PI, FRAC_PI_2};

use crate::model::{Car, Situation, Roundabout, Position};
use crate::physics::MAX_STEER;

pub trait Thinker {
    fn think(&mut self, situation: Situation);
}

impl Thinker for Car {

    fn think(&mut self, situatuion: Situation) {
        let desired_position = self.closest_position_on_roundabout(situatuion.roundabout);

        let (cx, cy) = &desired_position.coordinates;
        let (ax, ay) = &self.position.coordinates;

        let mut angle_to_position = (cy-ay).atan2(cx-ax) - self.position.orientation;
        angle_to_position -= 2.0*PI * ((angle_to_position + PI) / (2.0*PI)).floor();

        let distance_to_desired_position = ((cx-ax).powi(2) + (cy-ay).powi(2)).sqrt();

        let turing_radius = 1.0 / MAX_STEER  ;
        let _orientation_bias = (turing_radius - distance_to_desired_position).min(0.0) / turing_radius;

        self.desired_speed = 130.;
        self.desired_steer = angle_to_position.abs().max(FRAC_PI_4) / FRAC_PI_4 * MAX_STEER * angle_to_position.signum();

        self.debug.desired_position = Some(desired_position);
        self.debug.angle_to_desired = Some(angle_to_position);
    }
}

impl Car {
    fn closest_position_on_roundabout(&self, roundabout: &Roundabout) -> Position {
        let rel_x = self.position.coordinates.0 - roundabout.coordinates.0;
        let rel_y = self.position.coordinates.1 - roundabout.coordinates.1;

        let rel_mag = ( rel_x * rel_x + rel_y * rel_y ).sqrt();

        let distance = rel_mag - roundabout.radius;

        let angle = rel_y.atan2(rel_x);

        Position {
            coordinates: (
                self.position.coordinates.0 - distance * angle.cos(),
                self.position.coordinates.1 - distance * angle.sin()
            ),
            orientation: angle + FRAC_PI_2
        }
    }
}

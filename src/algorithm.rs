use std::f32::consts::{FRAC_PI_4,FRAC_PI_2, FRAC_PI_6};

use crate::model::{Car, Situation, Roundabout, Position};
use crate::physics::MAX_STEER;
use crate::util::normalize_angle;

pub trait Thinker {
    fn think(&mut self, situation: Situation);
}

impl Thinker for Car {

    fn think(&mut self, situatuion: Situation) {
        let target = situatuion.roundabout;
        let desired_position = self.closest_position_on_roundabout(target);
        let target_steering = 1.0 / target.radius;

        let turing_radius = 1.0 / MAX_STEER  ;
        let (cx, cy) = &desired_position.coordinates;
        let (ax, ay) = &self.position.coordinates;

        let distance_to_desired_position = ((cx-ax).powi(2) + (cy-ay).powi(2)).sqrt();

        let angle_to_position = normalize_angle((cy-ay).atan2(cx-ax) - self.position.orientation);
        let angle_to_orientation = normalize_angle(desired_position.orientation - self.position.orientation); 

        let orientation_bias = ((turing_radius*2.0 - distance_to_desired_position).max(0.0) / (turing_radius*2.0));
        dbg!(&orientation_bias);
        // TODO: experiment with this value, with how turing angle is decided

        let turning_angle = angle_to_position * (1.0 - orientation_bias)  +  angle_to_orientation * (orientation_bias);

        self.desired_speed = 130.;
        self.desired_steer = turning_angle.abs().min(FRAC_PI_4) / FRAC_PI_4 * MAX_STEER * turning_angle.signum() + target_steering * orientation_bias ;

        self.debug.desired_position = Some(desired_position);
        self.debug.angle_to_position = Some(angle_to_position);
        self.debug.angle_to_orientation = Some(angle_to_orientation);
        self.debug.turning_angle = Some(turning_angle);
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

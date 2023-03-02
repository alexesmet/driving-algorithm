use std::f32::consts::{FRAC_PI_4,FRAC_PI_2};

use crate::model::{Car, Position};
use crate::physics::MAX_STEER;
use crate::util::normalize_angle;

pub trait Thinker {
    fn think(&mut self);
}

impl Thinker for Car {

    fn think(&mut self) {
        let mut flag_to_swith_road = false;
        let road_node = self.navigator.get_road();
        self.brakes = false;
        match road_node {
            crate::model::Road::Turn { coordinates, radius, start_angle, end_angle, direction } => {

                let rel_x = self.position.coordinates.0 - coordinates.0;
                let rel_y = self.position.coordinates.1 - coordinates.1;
                let angle = rel_y.atan2(rel_x);
                let rel_mag = ( rel_x * rel_x + rel_y * rel_y ).sqrt();
                let distance = rel_mag - radius;
                let desired_position = Position {
                    coordinates: (
                        self.position.coordinates.0 - distance * angle.cos(),
                        self.position.coordinates.1 - distance * angle.sin()
                    ),
                    orientation: angle + match direction {
                        crate::model::RoadTurnDirection::CW => -FRAC_PI_2,
                        crate::model::RoadTurnDirection::CCW => FRAC_PI_2,
                    }
                };

                let target_steering = 1.0 / radius;

                let (cx, cy) = &desired_position.coordinates;
                let (ax, ay) = &self.position.coordinates;
                let distance_to_desired_position = ((cx-ax).powi(2) + (cy-ay).powi(2)).sqrt();

                let angle_to_position = normalize_angle((cy-ay).atan2(cx-ax) - self.position.orientation);
                let angle_to_orientation = normalize_angle(desired_position.orientation - self.position.orientation); 

                let orientation_bias = get_orientation_bias(distance_to_desired_position);
                let turning_angle = angle_to_position * (1.0 - orientation_bias)  +  angle_to_orientation * (orientation_bias);

                self.desired_speed = 130.;
                self.desired_steer = turning_angle.abs().min(FRAC_PI_4) / FRAC_PI_4 * MAX_STEER * turning_angle.signum() + target_steering * orientation_bias ;

                // mb rewrite using direction
                if angle > *end_angle || angle < *start_angle {
                    flag_to_swith_road = true;
                }

                self.debug.desired_position = Some(desired_position);
                self.debug.angle_to_position = Some(angle_to_position);
                self.debug.angle_to_orientation = Some(angle_to_orientation);
                self.debug.turning_angle = Some(turning_angle);
            },
            crate::model::Road::Line { start, end } => {
                let start_to_end = (end.0 - start.0 , end.1 - start.1);
                let start_to_me = (self.position.coordinates.0 - start.0 , self.position.coordinates.1 - start.1);
                let start_to_end_sq_mag = start_to_end.0.powi(2) + start_to_end.1.powi(2);
                let dot_product = start_to_me.0 * start_to_end.0 + start_to_me.1 * start_to_end.1;
                let normalized_distance = dot_product / start_to_end_sq_mag;
                let desired_position = Position {
                    coordinates: (
                        start.0 + start_to_end.0 * normalized_distance,
                        start.1 + start_to_end.1 * normalized_distance
                    ),
                    orientation: start_to_end.1.atan2(start_to_end.0)
                };

                let (cx, cy) = &desired_position.coordinates;
                let (ax, ay) = &self.position.coordinates;
                let distance_to_desired_position = ((cx-ax).powi(2) + (cy-ay).powi(2)).sqrt();

                let angle_to_position = normalize_angle((cy-ay).atan2(cx-ax) - self.position.orientation);
                let angle_to_orientation = normalize_angle(desired_position.orientation - self.position.orientation); 

                let orientation_bias = get_orientation_bias(distance_to_desired_position);
                let turning_angle = angle_to_position * (1.0 - orientation_bias)  +  angle_to_orientation * (orientation_bias);

                self.desired_speed = 130.;
                self.desired_steer = turning_angle.abs().min(FRAC_PI_4) / FRAC_PI_4 * MAX_STEER * turning_angle.signum();

                if normalized_distance > 1.0 {
                    flag_to_swith_road = true;
                }

                self.debug.desired_position = Some(desired_position);
                self.debug.angle_to_position = Some(angle_to_position);
                self.debug.angle_to_orientation = Some(angle_to_orientation);
                self.debug.turning_angle = Some(turning_angle);


            },
        }
        if flag_to_swith_road {
            self.navigator.switch_to_next_road();
        }
    }
}

fn get_orientation_bias(distance: f32) -> f32 {
    let turing_radius = 1.0 / MAX_STEER;
    (turing_radius*2.0 - distance).max(0.0) / (turing_radius*2.0)
}

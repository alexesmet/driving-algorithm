use std::f32::consts::{FRAC_PI_4,FRAC_PI_2};

use crate::model::{Car, Position, Road, RoadTurnDirection};
use crate::physics::{MAX_STEER, STEER_SPEED};
use crate::util::normalize_angle;

pub trait Thinker {
    fn think(&mut self);
}

impl Thinker for Car {

    fn think(&mut self) {
        let road = self.navigator.get_road();
        self.brakes = false;

        // find the closest position on the road
        let DesiredPosition { position: desired_position, on_road_end } = get_desired_position_on_a_road(&self.position, road);

        let distance_to_desired_position = ((self.position.coordinates.0-desired_position.coordinates.0).powi(2) +
                                            (self.position.coordinates.1-desired_position.coordinates.1).powi(2)).sqrt();

        // vector points from current self coordinates to closest pont on the road
        let angle_to_desired_position = (-self.position.coordinates.1+desired_position.coordinates.1)
                                  .atan2(-self.position.coordinates.0+desired_position.coordinates.0);

        // how should car hold the steering when it's precisely on the road.
        // Depends on:
        // - form of the road (primary)
        // - form of next part of road
        // - distance to next part of road
        let taget_steering = get_taget_steering(self, road);

        // this relative angle points towards clothest point on a road
        let angle_to_position = normalize_angle(angle_to_desired_position - self.position.orientation);

        // this relative angle is absolute target steering
        let angle_to_orientation = normalize_angle(desired_position.orientation - self.position.orientation); 

        // car ai chooses between two angles
        let orientation_bias = get_orientation_bias(distance_to_desired_position);
        let turning_angle = angle_to_position * (1.0 - orientation_bias)  +  angle_to_orientation * (orientation_bias);

        self.desired_speed = 170.;
        self.desired_steer = turning_angle.abs().min(FRAC_PI_4) / FRAC_PI_4 * MAX_STEER * turning_angle.signum() + taget_steering * orientation_bias ;

        self.debug.desired_position = Some(desired_position);
        self.debug.angle_to_position = Some(angle_to_position);
        self.debug.angle_to_orientation = Some(angle_to_orientation);
        self.debug.turning_angle = Some(turning_angle);

        if on_road_end {
            self.navigator.switch_to_next_road();
        }
    }
}

struct DesiredPosition {
    position: Position,
    on_road_end: bool
}

fn get_desired_position_on_a_road(position: &Position, road: &Road) -> DesiredPosition {
    match road {
        crate::model::Road::Turn { coordinates, radius, start_angle, end_angle, direction } => {
            let rel_x = position.coordinates.0 - coordinates.0;
            let rel_y = position.coordinates.1 - coordinates.1;
            let angle = rel_y.atan2(rel_x);
            let rel_mag = ( rel_x * rel_x + rel_y * rel_y ).sqrt();
            let distance = rel_mag - radius;
            DesiredPosition { position: Position {
                coordinates: (
                    position.coordinates.0 - distance * angle.cos(),
                    position.coordinates.1 - distance * angle.sin()
                ),
                orientation: angle + match direction {
                    RoadTurnDirection::CW => -FRAC_PI_2,
                    RoadTurnDirection::CCW => FRAC_PI_2,
                }
            }, on_road_end: match direction {
                RoadTurnDirection::CW =>  *end_angle   > angle && angle > *start_angle,
                RoadTurnDirection::CCW => *start_angle > angle && angle > *end_angle,
            }}
        },
        crate::model::Road::Line { start, end } => {
            let start_to_end = (end.0 - start.0 , end.1 - start.1);
            let start_to_me = (position.coordinates.0 - start.0 , position.coordinates.1 - start.1);
            let start_to_end_sq_mag = start_to_end.0.powi(2) + start_to_end.1.powi(2);
            let dot_product = start_to_me.0 * start_to_end.0 + start_to_me.1 * start_to_end.1;
            let normalized_distance = dot_product / start_to_end_sq_mag;
            DesiredPosition { position: Position {
                coordinates: (
                    start.0 + start_to_end.0 * normalized_distance,
                    start.1 + start_to_end.1 * normalized_distance
                ),
                orientation: start_to_end.1.atan2(start_to_end.0)
            }, on_road_end: normalized_distance > 1.0 }
        },
    }
}

fn get_taget_steering(car: &Car, road: &Road) -> f32 {
    let original_target_steering = get_target_steering_for_road(road);
    let future_target_steering = get_target_steering_for_road(car.navigator.get_next_road());

    let mean_between_original_and_future = (future_target_steering + original_target_steering) / 2.0;
    let diff_to_future = mean_between_original_and_future - original_target_steering;
    let distance_to_reach_half_of_future_steering = (car.speed * diff_to_future / STEER_SPEED).abs();

    let end = road.get_end();
    let distance_to_road_end = ((car.position.coordinates.0 - end.0).powi(2) + (car.position.coordinates.1 - end.1).powi(2)).sqrt();

    if distance_to_road_end > distance_to_reach_half_of_future_steering {
        original_target_steering
    } else {
        mean_between_original_and_future
    }
}

fn get_target_steering_for_road(road: &Road) -> f32 {
    match road {
        crate::model::Road::Turn { radius, direction, .. } => match direction {
            RoadTurnDirection::CW => -1.0 / radius,
            RoadTurnDirection::CCW => 1.0 / radius,
        }
        ,
        crate::model::Road::Line { .. } => 0.0,
    }
}

fn get_orientation_bias(distance: f32) -> f32 {
    let turing_radius = 1.0 / MAX_STEER;
    (turing_radius*2.0 - distance).max(0.0) / (turing_radius*2.0)
}

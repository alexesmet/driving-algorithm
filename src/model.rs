use std::fmt::Debug;
use std::f32::consts::FRAC_PI_2;

use crate::navigator::Navigator;



#[derive(Debug)]
pub struct Position {
    pub coordinates: (f32,f32),
    pub orientation: f32
}

#[derive(Debug)]
pub struct CarDebugInfo {
    pub desired_position: Option<Position>,
    pub angle_to_position: Option<f32>,
    pub angle_to_orientation: Option<f32>,
    pub turning_angle: Option<f32>,
}

impl Default for CarDebugInfo {
    fn default() -> Self {
        Self { desired_position: None, angle_to_position: None, angle_to_orientation: None, turning_angle: None }
    }
}

#[derive(Debug)]
pub struct Car {

    pub brakes: bool,
    pub desired_speed: f32,
    pub desired_steer: f32,

    pub speed: f32, 
    pub steer: f32,
    
    pub position: Position,

    pub navigator: Navigator,

    pub debug: CarDebugInfo,
}

impl Car {
    pub fn from_navigator(navigator: Navigator) -> Self {
        let start = navigator.get_road().get_start();

        Self { 
            brakes: false,
            desired_speed: 0.0,
            desired_steer: 0.0,
            navigator,

            speed: 0.0,
            steer: 0.0,

            position: start,

            debug: Default::default()
        }
    }
}


pub struct Roundabout {
    pub coordinates: (f32, f32),
    pub radius: f32
}




#[derive(Debug)]
pub enum RoadTurnDirection {
    CW, CCW
}

#[derive(Debug)]
pub enum Road {
    Turn { coordinates: (f32,f32), radius: f32, start_angle: f32, end_angle: f32, direction: RoadTurnDirection },
    Line { start: (f32,f32), end: (f32,f32) }
}

impl Road {
    pub fn get_start(&self) -> Position {
        match self {
            Road::Turn { coordinates, radius, start_angle, direction, .. } => {
                let x = coordinates.0 + radius * start_angle.cos();
                let y = coordinates.1 + radius * start_angle.sin();
                let a = start_angle + match direction {
                    RoadTurnDirection::CW => -FRAC_PI_2,
                    RoadTurnDirection::CCW => FRAC_PI_2,
                };
                return Position { coordinates: (x, y), orientation: a };
            },
            Road::Line { start, end } => {
                return Position { coordinates: start.clone(), orientation: (end.1 - start.1).atan2(end.0 - start.0) }
            },
        }
        
    }
    pub fn get_end(&self) -> (f32,f32) {
        match self {
            Road::Turn { coordinates, radius, end_angle, .. } => {
                (coordinates.0 + radius * end_angle.cos(), coordinates.1 + radius * end_angle.sin())
            },
            Road::Line { end, .. } => {
                end.clone()
            },
        }
    }
}


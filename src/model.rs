use std::{rc::Rc, cell::RefCell, fmt::Debug};



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
    pub road_node: RoadNodeRef,

    pub speed: f32, 
    pub steer: f32,
    
    pub position: Position,

    pub debug: CarDebugInfo,
}

impl Default for Car {
    fn default() -> Self {
        Self { 
            brakes: false,
            desired_speed: 0.0,
            desired_steer: 0.0,
            road_node: None,

            speed: 0.0,
            steer: 0.0,

            position: Position {
                coordinates: (0.0, 0.0), 
                orientation: 0.0 
            },

            debug: Default::default()
        }
    }
}


pub struct Roundabout {
    pub coordinates: (f32, f32),
    pub radius: f32
}


#[derive(Debug)]
pub enum Road {
    // Straight { start: (f32,f32), end: (f32,f32) },
    Turn { coordinates: (f32,f32), radius: f32, start_angle: f32, end_angle: f32 }
}

type RoadNodeRef = Option<Rc<RefCell<RoadNode>>>;

pub struct RoadNode {
    pub road: Road,
    pub next: RoadNodeRef
}

impl Debug for RoadNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RoadNode").field("road", &self.road).field("next", &"<hidden>").finish()
    }
}





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

#[derive(Debug)]
pub struct Car {

    pub brakes: bool,
    pub desired_speed: f32,
    pub desired_steer: f32,

    pub speed: f32, 
    pub steer: f32,
    
    pub position: Position,

    pub debug: CarDebugInfo,
}

pub struct Situation<'a> {
    pub roundabout: &'a Roundabout
}

pub struct Roundabout {
    pub coordinates: (f32, f32),
    pub radius: f32
}

impl Default for Car {
    fn default() -> Self {
        Self { 
            brakes: false,
            desired_speed: 0.0,
            desired_steer: 0.0,

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

impl Default for CarDebugInfo {
    fn default() -> Self {
        Self { desired_position: None, angle_to_position: None, angle_to_orientation: None, turning_angle: None }
    }
}



pub struct Car {

    pub speed: f32, 
    pub steer: f32,

    pub coordinates: (f32,f32),
    pub orientation: f32
}

impl Default for Car {
    fn default() -> Self {
        Self { 
            speed: 0.0,
            steer: 0.0,

            coordinates: (0.0, 0.0), 
            orientation: 0.0 
        }
    }
}

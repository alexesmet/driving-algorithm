use std::f32::consts::PI;

pub fn normalize_angle(a: f32) -> f32 {
    a - 2.0*PI * ((a + PI) / (2.0*PI)).floor()
}

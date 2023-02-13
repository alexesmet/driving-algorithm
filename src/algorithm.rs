use crate::model::{Car, Situation, Roundabout, Position};

pub trait Thinker {
    fn think(&mut self, situation: Situation);
}

impl Thinker for Car {

    fn think(&mut self, situatuion: Situation) {
        self.debug.desired_position = Some(self.closest_position_on_roundabout(situatuion.roundabout));

        self.desired_speed = 130.;
        self.desired_steer = 0.01;
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
            orientation: angle + std::f32::consts::FRAC_PI_2
        }
    }
}

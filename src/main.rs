
mod model;
mod drawing;
mod physics;
mod algorithm;

use algorithm::Thinker;
use model::{Situation, Roundabout};
use nannou::prelude::*;
use drawing::{Drawing, DrawingDebug};
use physics::Physics;


fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    car: model::Car,
    road: model::Roundabout
}

fn model(_app: &App) -> Model {
    Model { 
        car: model::Car::default(),
        road: Roundabout { coordinates: (0., 0.), radius: 100. }
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.car.think(Situation { roundabout: &model.road });
    model.car.update();
}

fn view(app: &App, model: &Model, frame: Frame){
    
    // Prepare to draw.
    let draw = app.draw();

    draw.background().color(WHITESMOKE);

    model.car.draw_debug(&draw);

    model.road.draw(&draw);
    model.car.draw(&draw);
    
    draw.to_frame(app, &frame).unwrap();
}

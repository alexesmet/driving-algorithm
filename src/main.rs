
mod model;
mod drawing;
mod physics;
mod algorithm;

use algorithm::Thinker;
use nannou::prelude::*;
use drawing::Drawing;
use physics::Physics;


fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    car: model::Car
}

fn model(_app: &App) -> Model {
    Model { car: model::Car::default() }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.car.think();
    model.car.update();
}

fn view(app: &App, model: &Model, frame: Frame){
    
    // Prepare to draw.
    let draw = app.draw();

    draw.background().color(WHITESMOKE);

    model.car.draw(&draw);
    
    draw.to_frame(app, &frame).unwrap();
}

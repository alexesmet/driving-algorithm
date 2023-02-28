
mod model;
mod drawing;
mod physics;
mod algorithm;
mod util;
mod navigator;

use std::{f32::consts::FRAC_PI_2, rc::Rc, cell::RefCell};

use algorithm::Thinker;
use model::Road;
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
}

fn model(_app: &App) -> Model {

    let road_1 = Road::Turn { coordinates: ( 100., 100.), radius: (100.), start_angle: ( 0.), end_angle: ( FRAC_PI_2) };
    let road_2 = Road::Turn { coordinates: ( 100.,-100.), radius: (300.), start_angle: ( FRAC_PI_2), end_angle: ( PI) };
    let road_3 = Road::Turn { coordinates: (-100.,-100.), radius: (100.), start_angle: (-PI), end_angle: (-FRAC_PI_2) };
    let road_4 = Road::Turn { coordinates: (-100., 100.), radius: (300.), start_angle: (-FRAC_PI_2), end_angle:  (0.) };

    Model { 
        car: todo!()
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.car.think();
    model.car.update();
}

fn view(app: &App, model: &Model, frame: Frame){
    
    // Prepare to draw.
    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();

    draw.background().color(WHITESMOKE);

    model.car.draw_debug(&draw);
    model.car.draw(&draw);

    let pad = 6.0;
    let car_debug = format!("{:#?}", model.car);
    draw.text(&car_debug)
            .h(win.pad(pad).h())
            .w(win.pad(pad).w())
            .line_spacing(pad)
            .align_text_bottom()
            .color(GRAY)
            .left_justify();

    draw.to_frame(app, &frame).unwrap();
}

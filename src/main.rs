mod model;
mod drawing;
mod physics;
mod algorithm;
mod util;
mod navigator;

use std::{f32::consts::FRAC_PI_2, rc::Rc};

use algorithm::Thinker;
use model::{Road, Car};
use nannou::prelude::*;
use drawing::{Drawing, DrawingDebug};
use physics::Physics;
use navigator::{RoadMap,RoadNode,Navigator};


fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    map: Rc<RoadMap>,
    car: model::Car,
    debug: bool
}

fn model(_app: &App) -> Model {

    let args: Vec<String> = std::env::args().collect(); // I hate to put it here...
    let debug = args.iter().any(|s| s == "--debug");

    let road_1 = Road::Turn { coordinates: ( 100., 100.), radius: (100.), start_angle: ( 0.), end_angle: ( FRAC_PI_2), direction: model::RoadTurnDirection::CCW };
    let road_2 = Road::Turn { coordinates: ( 100.,-100.), radius: (300.), start_angle: ( FRAC_PI_2), end_angle: ( PI), direction: model::RoadTurnDirection::CCW };
    let road_3 = Road::Turn { coordinates: (-100.,-100.), radius: (100.), start_angle: (-PI), end_angle: (-FRAC_PI_2), direction: model::RoadTurnDirection::CCW };
    let road_4 = Road::Turn { coordinates: (-100., 100.), radius: (300.), start_angle: (-FRAC_PI_2), end_angle:  (0.), direction: model::RoadTurnDirection::CCW };

    let road_map = Rc::new(RoadMap::new(vec![
         RoadNode { road: road_1, /* 0 */ next: vec![1] },
         RoadNode { road: road_2, /* 1 */ next: vec![2] },
         RoadNode { road: road_3, /* 2 */ next: vec![3] },
         RoadNode { road: road_4, /* 3 */ next: vec![0] }
    ]).expect("Should have created RoadMap"));

    let navigator = Navigator::new(Rc::clone(&road_map), 0).expect("Should have created the navigator");

    Model { 
        map: road_map,
        car: Car::from_navigator(navigator),
        debug
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.car.think();
    model.car.update();
}

fn view(app: &App, model: &Model, frame: Frame){
    
    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();

    draw.background().color(WHITESMOKE);
    
    model.map.draw(&draw);

    if model.debug { model.car.draw_debug(&draw); }
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

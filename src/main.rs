
mod model;
mod drawing;
mod physics;
mod algorithm;
mod util;

use std::{f32::consts::FRAC_PI_2, rc::Rc, cell::RefCell};

use algorithm::Thinker;
use model::{Road, RoadNode};
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
    map: Vec<Rc<RefCell<RoadNode>>>
}

fn model(_app: &App) -> Model {

    let road_1 = Road::Turn { coordinates: ( 100., 100.), radius: (100.), start_angle: ( 0.), end_angle: ( FRAC_PI_2) };
    let road_2 = Road::Turn { coordinates: ( 100.,-100.), radius: (300.), start_angle: ( FRAC_PI_2), end_angle: ( PI) };
    let road_3 = Road::Turn { coordinates: (-100.,-100.), radius: (100.), start_angle: (-PI), end_angle: (-FRAC_PI_2) };
    let road_4 = Road::Turn { coordinates: (-100., 100.), radius: (300.), start_angle: (-FRAC_PI_2), end_angle:  (0.) };
    
    let node_4 = Rc::new(RefCell::new( RoadNode { road: road_4, next: None }));
    let node_3 = Rc::new(RefCell::new( RoadNode { road: road_3, next: Some(node_4.clone()) }));
    let node_2 = Rc::new(RefCell::new( RoadNode { road: road_2, next: Some(node_3.clone()) }));
    let node_1 = Rc::new(RefCell::new( RoadNode { road: road_1, next: Some(node_2.clone()) }));
    
    node_4.borrow_mut().next = Some(node_1.clone());
    
    let mut car = model::Car::default();
    car.road_node = Some(node_4.clone());

    Model { 
        car,
        map: vec![node_1.clone(), node_2.clone(), node_3.clone(), node_4.clone()]
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

    model.map.iter().for_each(|r| r.borrow().road.draw(&draw));

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

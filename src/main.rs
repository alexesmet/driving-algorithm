mod model;
mod drawing;
mod physics;
mod algorithm;
mod util;
mod navigator;

use std::{rc::Rc, fs};

use algorithm::Thinker;
use model::Car;
use drawing::{Drawing, DrawingDebug};
use physics::Physics;
use navigator::{RoadMap,Navigator, road_nodes_from_toml};

use nannou::prelude::*;


fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    map: Rc<RoadMap>,
    cars: Vec<Car>,
    debug: bool
}

fn model(_app: &App) -> Model {

    let contents = fs::read_to_string("resources/map.toml").expect("Could not find map file");

    // TODO: Movce command line arguments, model initialization to fn main
    let args: Vec<String> = std::env::args().collect(); // I hate to put it here...
    let debug = args.iter().any(|s| s == "--debug");


    let map = Rc::new(road_nodes_from_toml(&contents).map_err(|e| { 
        use navigator::RoadMapDeserializationError::*; 
        eprint!("There was an error in the TOML file you provided as a map: ");
        match e {
            InvalidFormat { error } => eprintln!("TOML file has invalid format: {error}"),
            MissingField { path } => eprintln!("missing field: `{path}`"),
            InvalidFieldType { path } => eprintln!("field `{path}` is present, but it's value has a wrong type"),
            UnknowRoadType { path, value } => eprintln!("road type you provided on `{path}` is unknown: '{value}'"),
            InvalidFractionNotation { path, error } => {
                use navigator::FractionNotationError::*;
                eprint!("could not read fraction notation you provided on `{path}`: ");
                match error {
                    DivisionSymbolAbsent => eprintln!("fraction notation doesnt contain the `/` symbol"),
                    InvalidNumerator => eprintln!("fraction notation has invalid numerator"),
                    InvalidDenominator => eprintln!("fraction notation has invalid denominator"),
                }
            },
            InvalidDirectionNotation { path } => eprintln!("direction for road you provided on `{path}` is unknown"),
            UnknownRoadReferece { reference } => eprintln!("while building graph structure, detected reference for unknown node: '{reference}'"),
            RoadMapError { error } => {
                use navigator::RoadMapError::*;
                eprint!("file was parsed successfully, but the road structure is invalid: ");
                match error {
                    NoRoadsPresent => eprintln!("there are no roads in this map"),
                    DeadEndPresent => eprintln!("this map contains a dead end, every road should have a next one"),
                    NextIndexOutOfBounds => eprintln!("index of next road lies out of bounds of road list"),
                }
            },
        }
        std::process::exit(1);
    }).unwrap()); // FIXME: this unwrap is useless, will be gone when model building is moved to main

    // TODO: also move car to map file, to load road situations from file
    //
    let cars: Vec<Car> = vec![{
        let navigator = Navigator::new(Rc::clone(&map), 0).expect("Should have created the navigator");
        let car = Car::from_navigator(navigator);
        // car.position.coordinates = (80.0, -80.0);
        car
    }, {
        let navigator = Navigator::new(Rc::clone(&map), 4).expect("Should have created the navigator");
        let car = Car::from_navigator(navigator);
        //car.position.coordinates = (-80.0, 80.0);
        car
    }];

    Model { map, cars, debug }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for car in model.cars.iter_mut() {
        car.think();
        car.update();
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    
    let draw = app.draw();

    draw.background().color(WHITESMOKE);
    
    model.map.draw(&draw);

    model.cars.iter().for_each(|car| {
        if model.debug { car.draw_debug(&draw); }
        car.draw(&draw);
    });

    // let window = app.main_window();
    // let win = window.rect();
    // let pad = 6.0;
    // let car_debug = format!("{:#?}", model.car);
    // draw.text(&car_debug)
    //     .h(win.pad(pad).h())
    //     .w(win.pad(pad).w())
    //     .line_spacing(pad)
    //     .align_text_bottom()
    //     .color(GRAY)
    //     .left_justify();

    draw.to_frame(app, &frame).unwrap();
}

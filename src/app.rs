use clap::{Arg, App, PossibleValue};

use crate::render::{Render, NullRenderer, TextRenderer};
use crate::hanoi::{Disk, State};

pub fn configure_app() -> (State, Box<dyn Render>) {
    let matches = App::new("Hanoi Tower Solver")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
    
        .arg(Arg::new("disks")
            .short('d')
            .long("disks")
            .value_name("count")
            .help("Sets a count of disks (0; 128]")
            .takes_value(true)
            .validator(|s| s.parse::<Disk>().map(|x| 0 < x && x <= 128))
            .default_value("3"))
    
        .arg(Arg::new("render")
            .short('r')
            .long("render")
            .value_name("type")
            .help("Sets the type of render")
            .takes_value(true)
            .ignore_case(true)
            .possible_values([
                PossibleValue::new("text")
                    .help("Show moves and states in console"),
                PossibleValue::new("null")
                    .help("Do not show moves and states")])
            .default_value("text")
            .default_missing_value("graphic"))
            
        .get_matches();

    let state = match matches.value_of("disks") {
        Some(s) => State::new(s.parse().unwrap()).unwrap(),
        _ => Default::default(),
    };

    let render: Box<dyn Render> = match matches.value_of("render") {
        Some("text")    => Box::new(TextRenderer::new()),
        Some("null")    => Box::new(NullRenderer::new()),
        _ => panic!("Unknown type of render"),
    };

    (state, render)
}
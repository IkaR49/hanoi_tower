use std::io;
use clap::{Arg, App, PossibleValue};

mod hanoi;
mod render;

use render::{Render, NullRenderer, TextRenderer};
use hanoi::{Disk, State, Move};

fn main() -> io::Result<()> {
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

    let mut state = match matches.value_of("disks") {
        Some(s) => State::new(s.parse().unwrap())?,
        _ => Default::default(),
    };

    let mut render: Box<dyn Render> = match matches.value_of("render") {
        Some("text")    => Box::new(TextRenderer::new()),
        Some("null")    => Box::new(NullRenderer::new()),
        _ => panic!("Unknown type of render"),
    };

    // Счётсик количества итераций
    let mut counter: u128 = 0;

    // Отобразим начальное состояние
    render.render_state(&state);

    // Рассчитаем первый шаг
    let mut last_move = Move::default().next_for(&state);

    while let Some(mv) = last_move {
        // Переместим диск
        state.move_disk(&mv)?;

        // Отобразим новое состояние и выполненное перемещение
        render.render(&mv, &state);

        // Рассчитаем следующие перемещение
        last_move = mv.next_for(&state);

        counter += 1;
    }

    println!("Finish with {} steps", counter);
    Ok(())
}

mod hanoi;
mod render;
mod app;

use std::io;
use hanoi::Move;
use app::configure_app;

fn main() -> io::Result<()> {
    let (mut state, mut render) = configure_app();

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

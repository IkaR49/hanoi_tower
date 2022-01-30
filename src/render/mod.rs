use crate::hanoi::{State, Move};

pub trait Render {
    fn render_state(&mut self, state: &State);
    fn render_move(&mut self, mv: &Move);
    fn render(&mut self, mv: &Move, state: &State) {
        self.render_move(mv);
        self.render_state(state);
    }
}

mod null;
mod text;

pub use null::NullRenderer;
pub use text::TextRenderer;

use crate::render::Render;
use crate::hanoi::{State, Move};

pub struct TextRenderer {}
impl TextRenderer {
    pub const fn new() -> Self { Self {} }
}
impl Render for TextRenderer {
    fn render_state(&mut self, state: &State) {
        println!("{}", state);
    }
    fn render_move(&mut self, mv: &Move) {
        println!("Do: {}", mv);
    }
}

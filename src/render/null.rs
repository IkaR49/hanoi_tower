use crate::render::Render;
use crate::hanoi::{State, Move};

pub struct NullRenderer {}
impl NullRenderer {
    pub const fn new() -> Self { Self {} }
}
impl Render for NullRenderer {
    fn render_state(&mut self, _: &State) {}
    fn render_move(&mut self, _: &Move) {}
    fn render(&mut self, _: &Move, _: &State) {}
}

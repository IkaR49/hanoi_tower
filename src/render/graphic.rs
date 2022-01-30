#![allow(unused)]

use crate::render::Render;
use crate::hanoi::{State, Move};

pub struct GraphicRenderer {}
impl GraphicRenderer {
    pub const fn new() -> Self { Self {} }
}
impl Render for GraphicRenderer {
    fn render_state(&mut self, state: &State) {
    }
    fn render_move(&mut self, mv: &Move) {
    }
}

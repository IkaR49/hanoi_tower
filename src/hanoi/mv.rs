use std::fmt;
use super::{Rod, State};

#[derive(Clone)]
pub struct Move {
    pub from: Rod,
    pub to  : Rod,
}

impl Move {
    pub fn next_for(self, state: &State) -> Option<Self> {
        let (rod0, rod1) = match self.to {
            Rod::Rod1 => (Rod::Rod2, Rod::Rod3),
            Rod::Rod2 => (Rod::Rod1, Rod::Rod3),
            Rod::Rod3 => (Rod::Rod1, Rod::Rod2),
        };
        let top0 = state.top_on(&rod0);
        let top1 = state.top_on(&rod1);

        if top0 == top1 {
            return None;
        }

        let from = if top0 > top1 { rod0 } else { rod1 };
        let top = state.top_on(&from);
        let to = if top % 2 == 0 {
            from.clone().clockwise() // Чётное двигаем по часовой стрелке
        } else {
            from.clone().counter_clockwise() // Нечётное двигаем против часовой стрелки
        };

        Some(Move { from, to })
    }
}

impl Default for Move {
    fn default() -> Self {
        Move { from: Rod::Rod1, to: Rod::Rod2 }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} --> {}", self.from, self.to)
    }
}

use std::fmt;

#[derive(Clone)]
pub enum Rod {
    Rod1 = 0,
    Rod2 = 1,
    Rod3 = 2,
}

impl Rod {
    pub const fn count() -> usize {
        3
    }
    pub const fn clockwise(self) -> Rod {
        match self {
            Rod::Rod1 => Rod::Rod2,
            Rod::Rod2 => Rod::Rod3,
            Rod::Rod3 => Rod::Rod1,
        }
    }
    pub const fn counter_clockwise(self) -> Rod {
        match self {
            Rod::Rod1 => Rod::Rod3,
            Rod::Rod2 => Rod::Rod1,
            Rod::Rod3 => Rod::Rod2,
        }
    }
}

impl fmt::Display for Rod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rod{}", self.clone() as usize + 1)
    }
}

impl From<Rod> for usize {
    fn from(rod: Rod) -> Self {
        rod as Self
    }
}

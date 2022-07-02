#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    DOWN,
    LEFT,
    RIGHT,
    UP,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

impl Pos {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn moved(&self, direction: Direction) -> Self {
        match direction {
            Direction::DOWN => Self::new(self.x, self.y + 1),
            Direction::LEFT => Self::new(self.x - 1, self.y),
            Direction::RIGHT => Self::new(self.x + 1, self.y),
            Direction::UP => Self::new(self.x, self.y - 1),
        }
    }
}

impl PartialEq<(u32, u32)> for Pos {
    fn eq(&self, rhs: &(u32, u32)) -> bool {
        self.x == rhs.0 && self.y == rhs.1
    }
}

impl Into<(u32, u32)> for Pos {
    fn into(self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl From<(u32, u32)> for Pos {
    fn from(other: (u32, u32)) -> Self {
        Self::new(other.0, other.1)
    }
}

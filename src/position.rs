use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position { x: x, y: y }
    }

    pub fn origin() -> Position {
        Position { x: 0.0, y: 0.0 }
    }

    pub fn min(&self, other: &Position) -> Position {
        Position {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(&self, other: &Position) -> Position {
        Position {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn min_max(&self, min_max: &(Position, Position)) -> (Position, Position) {
        (self.min(&min_max.0), self.max(&min_max.1))
    }
}

impl Add<Position> for Position {
    type Output = Position;
    fn add(self, other: Position) -> Self::Output {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

use super::{pmin, pmax, Position};

pub struct Bounds {
    min_max: Option<(Position, Position)>,
}

impl Bounds {
    pub fn new() -> Bounds {
        Bounds { min_max: None }
    }

    pub fn extend(&mut self, bounds: Bounds) {
        if let Some((a, b)) = bounds.min_max {
            self.add_position(a);
            self.add_position(b);
        }
    }

    pub fn add_position(&mut self, pos: Position) {
        let mm = match self.min_max {
            None => (pos, pos),
            Some((ref min, ref max)) => (pmin(&pos, min), pmax(&pos, max)),
        };

        self.min_max = Some(mm);
    }

    pub fn width(&self) -> f32 {
        let (min, max) = self.min_max.unwrap();
        (max.x - min.x).abs()
    }

    pub fn height(&self) -> f32 {
        let (min, max) = self.min_max.unwrap();
        (max.y - min.y).abs()
    }

    pub fn min(&self) -> Option<Position> {
        self.min_max.map(|m| m.0)
    }

    pub fn max(&self) -> Option<Position> {
        self.min_max.map(|m| m.1)
    }

    pub fn min_x(&self) -> f32 {
        self.min().unwrap().x
    }

    pub fn min_y(&self) -> f32 {
        self.min().unwrap().y
    }

    pub fn max_x(&self) -> f32 {
        self.max().unwrap().x
    }

    pub fn max_y(&self) -> f32 {
        self.max().unwrap().y
    }
}

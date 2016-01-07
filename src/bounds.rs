use super::position::Position;

pub struct Bounds {
    min_max: Option<(Position, Position)>,
}

impl Bounds {
    pub fn new() -> Bounds {
        Bounds { min_max: None }
    }

    pub fn add_position(&mut self, pos: Position) {
        let mm = match self.min_max {
            None => (pos, pos),
            Some(ref a) => pos.min_max(a),
        };

        self.min_max = Some(mm);
    }

    pub fn is_bounded(&self) -> bool {
        self.min_max.is_some()
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
extern crate eps_writer;

use std::fs::File;
use eps_writer::{EpsDocument, Line, Position, Point};

fn main() {
    let mut document = EpsDocument::new();
    document.add_shape(Box::new(Line(Position::new(0.0, 0.0), Position::new(1.0, 0.0))));
    document.add_shape(Box::new(Line(Position::new(1.0, 0.0), Position::new(1.0, 1.0))));
    document.add_shape(Box::new(Line(Position::new(1.0, 1.0), Position::new(0.0, 1.0))));
    document.add_shape(Box::new(Line(Position::new(0.0, 1.0), Position::new(0.0, 0.0))));

    document.add_shape(Box::new(Point(Position::new(0.5, 0.5), 0.25)));
    let mut file = File::create("test.eps").unwrap();
    document.write_eps(&mut file, 100.0, 100.0).unwrap();
}

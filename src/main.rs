extern crate eps_writer;

use std::fs::File;
use eps_writer::{EpsDocument, Line, Position};

fn main() {
    let mut document = EpsDocument::new();
    document.add_shape(Box::new(Line(Position::new(0.0, 0.0), Position::new(1.0, 1.0))));
    let mut file = File::create("blah.eps").unwrap();
    document.write_eps(&mut file, 100.0, 100.0).unwrap();
}

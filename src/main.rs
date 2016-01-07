use std::io::{self, Write};
use std::fs::File;

pub use position::Position;
use bounds::Bounds;

pub mod position;
mod bounds;

trait Shape {
    fn bounding_box(&self) -> (Position, Position);
    fn write_eps(&self, wr: &mut Write) -> io::Result<()>;
}

pub struct Point(pub Position);
pub struct Line(pub Position, pub Position);

impl Shape for Point {
    fn bounding_box(&self) -> (Position, Position) {
        (self.0, self.0)
    }
    fn write_eps(&self, wr: &mut Write) -> io::Result<()> {
        Ok(())
    }
}

impl Shape for Line {
    fn bounding_box(&self) -> (Position, Position) {
        (self.0.min(&self.1), self.0.max(&self.0))
    }

    fn write_eps(&self, wr: &mut Write) -> io::Result<()> {
        writeln!(wr,
                 "newpath {} {} moveto {} {} lineto stroke",
                 self.0.x,
                 self.0.y,
                 self.1.x,
                 self.1.y)
    }
}

struct EpsDocument {
    shapes: Vec<Box<Shape>>,
}

impl EpsDocument {
    pub fn new() -> EpsDocument {
        EpsDocument { shapes: Vec::new() }
    }

    pub fn add_shape(&mut self, shape: Box<Shape>) {
        self.shapes.push(shape);
    }

    /// Write as Embedded Postscript (EPS)
    pub fn write_eps(&self, wr: &mut Write, min_width: f32, min_height: f32) -> io::Result<()> {
        // Determine extend of canvas
        let mut bounds = Bounds::new();

        for shape in &self.shapes {
            let (min, max) = shape.bounding_box();
            bounds.add_position(min);
            bounds.add_position(max);
        }

        let width = bounds.width().max(min_width);
        let height = bounds.height().max(min_height);
        let border_percent = 0.1;

        let scale = 1.0 + 2.0 * border_percent;

        try!(writeln!(wr,
                      r#"%%!PS-Adobe-3.0 EPSF-3.0
%%Creator: https://github.com/mneumann/eps-writer-rs
%%DocumentData: Clean7Bit
%%Origin: 0 0
%%BoundingBox: {} {} {} {}
%%LanguageLevel: 2
%%Pages: 1
%%Page: 1 1
"#,
                      bounds.min_x() - border_percent * width,
                      bounds.min_y() - border_percent * height,
                      bounds.max_x() + border_percent * width,
                      bounds.max_y() + border_percent * height));

        // use a stroke width of 0.1% of the width or height of the canvas
        let stroke_width = scale * width.max(height) / 1000.0;
        try!(writeln!(wr, r#"{} setlinewidth"#, stroke_width));

        for shape in &self.shapes {
            try!(shape.write_eps(wr));
        }

        writeln!(wr, "%%EOF")
    }
}

fn main() {
    let mut document = EpsDocument::new();
    document.add_shape(Box::new(Line(Position::new(0.0, 0.0), Position::new(1.0, 1.0))));
    let mut file = File::create("blah.eps").unwrap();
    document.write_eps(&mut file, 100.0, 100.0).unwrap();
}

use std::io::{self, Write};

pub use position::Position;
use bounds::Bounds;

pub mod position;
mod bounds;

pub trait Shape {
    fn bounds(&self) -> Bounds;
    fn write_eps(&self, wr: &mut Write) -> io::Result<()>;
}

pub struct Point(pub Position, pub f32);
pub struct Points(pub Vec<Position>, pub f32);
pub struct Line(pub Position, pub Position);

impl Shape for Point {
    fn bounds(&self) -> Bounds {
        let mut b = Bounds::new();
        b.add_position(self.0);
        b
    }

    fn write_eps(&self, wr: &mut Write) -> io::Result<()> {
        writeln!(wr,
                 "newpath {} {} {} 0 360 arc fill",
                 self.0.x,
                 self.0.y,
                 self.1)
    }
}

impl Shape for Points {
    fn bounds(&self) -> Bounds {
        let mut b = Bounds::new();
        for &pos in &self.0 {
            b.add_position(pos);
        }
        b
    }

    fn write_eps(&self, wr: &mut Write) -> io::Result<()> {
        try!(writeln!(wr,
                      "/p {{
2 dict begin
/y exch def
/x exch def
gsave
newpath x y {} 0 360 \
                       arc fill
grestore
end
}} def",
                      self.1));
        for pos in &self.0 {
            try!(writeln!(wr, "{:.4} {:.4} p", pos.x, pos.y));
        }

        Ok(())
    }
}

impl Shape for Line {
    fn bounds(&self) -> Bounds {
        let mut b = Bounds::new();
        b.add_position(self.0);
        b.add_position(self.1);
        b
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

pub struct EpsDocument {
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
            bounds.extend(shape.bounds());
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

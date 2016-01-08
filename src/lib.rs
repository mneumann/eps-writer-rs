extern crate nalgebra as na;

use std::io::{self, Write};
pub use bounds::Bounds;
pub use na::Vec2;

pub mod bounds;

pub type Position = na::Pnt2<f32>;

fn pmin(p1: Position, p2: Position) -> Position {
    Position::new(p1.x.min(p2.x), p1.y.min(p2.y))
}

fn pmax(p1: Position, p2: Position) -> Position {
    Position::new(p1.x.max(p2.x), p1.y.max(p2.y))
}

fn ptransform(p: Position, translation: Vec2<f32>, scale: Vec2<f32>) -> Position {
    let n = p + translation;
    Position::new(n.x * scale.x, n.y * scale.y)
}

pub trait Shape {
    fn visit_pos(&self, cb: &mut FnMut(&Position));
    fn visit_pos_mut(&mut self, cb: &mut FnMut(&mut Position));

    fn bounds(&self) -> Bounds {
        let mut b = Bounds::new();
        self.visit_pos(&mut |p| b.add_position(*p));
        b
    }

    fn transform(&mut self, translation: Vec2<f32>, scale: Vec2<f32>) {
        self.visit_pos_mut(&mut |p| *p = ptransform(*p, translation, scale));
    }

    fn write_eps(&self, wr: &mut Write) -> io::Result<()>;
}

pub struct SetRGB(pub f32, pub f32, pub f32);
pub struct Point(pub Position, pub f32);
pub struct Points(pub Vec<Position>, pub f32);
pub struct Line(pub Position, pub Position);
pub struct Lines(pub Vec<(Position, Position)>);
pub struct PolyLine(pub Vec<Position>);

impl Shape for SetRGB {
    fn visit_pos(&self, _cb: &mut FnMut(&Position)) {}
    fn visit_pos_mut(&mut self, _cb: &mut FnMut(&mut Position)) {}

    fn write_eps(&self, wr: &mut Write) -> io::Result<()> {
        writeln!(wr, "{:.2} {:.2} {:.2} setrgbcolor", self.0, self.1, self.2)
    }
}

impl Shape for Point {
    fn visit_pos(&self, cb: &mut FnMut(&Position)) {
        cb(&self.0);
    }
    fn visit_pos_mut(&mut self, cb: &mut FnMut(&mut Position)) {
        cb(&mut self.0);
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
    fn visit_pos(&self, cb: &mut FnMut(&Position)) {
        for p in &self.0 {
            cb(p);
        }
    }
    fn visit_pos_mut(&mut self, cb: &mut FnMut(&mut Position)) {
        for p in &mut self.0 {
            cb(p);
        }
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

impl Shape for Lines {
    fn visit_pos(&self, cb: &mut FnMut(&Position)) {
        for &(ref p1, ref p2) in &self.0 {
            cb(p1);
            cb(p2);
        }
    }
    fn visit_pos_mut(&mut self, cb: &mut FnMut(&mut Position)) {
        for &mut (ref mut p1, ref mut p2) in &mut self.0 {
            cb(p1);
            cb(p2);
        }
    }

    fn write_eps(&self, wr: &mut Write) -> io::Result<()> {
        try!(writeln!(wr,
                      "/l {{
2 dict begin
/y2 exch def
/x2 exch def
/y1 exch def
/x1 exch def
\
                       gsave
newpath x1 y1 moveto x2 y2 lineto stroke
grestore
end
}} def"));
        for &(p1, p2) in &self.0 {
            try!(writeln!(wr, "{:.4} {:.4} {:.4} {:.4} l", p1.x, p1.y, p2.x, p2.y));
        }

        Ok(())
    }
}

impl Shape for Line {
    fn visit_pos(&self, cb: &mut FnMut(&Position)) {
        cb(&self.0);
        cb(&self.1);
    }
    fn visit_pos_mut(&mut self, cb: &mut FnMut(&mut Position)) {
        cb(&mut self.0);
        cb(&mut self.1);
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

impl Shape for PolyLine {
    fn visit_pos(&self, cb: &mut FnMut(&Position)) {
        for p in &self.0 {
            cb(p);
        }
    }
    fn visit_pos_mut(&mut self, cb: &mut FnMut(&mut Position)) {
        for p in &mut self.0 {
            cb(p);
        }
    }

    fn write_eps(&self, wr: &mut Write) -> io::Result<()> {
        match self.0.split_first() {
            Some((first, tail)) if tail.len() > 0 => {
                try!(write!(wr, "newpath {:.4} {:.4} moveto ", first.x, first.y));
                for l in tail {
                    try!(write!(wr, "{:.4} {:.4} lineto ", l.x, l.y));
                }
                try!(writeln!(wr, "stroke"));
            }
            _ => {}
        }
        Ok(())
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

    pub fn get_bounds(&self) -> Bounds {
        // Determine extend of canvas
        let mut bounds = Bounds::new();

        for shape in &self.shapes {
            bounds.extend(shape.bounds());
        }

        bounds
    }

    pub fn transform(&mut self, translate: Vec2<f32>, scale: Vec2<f32>) {
        for shape in self.shapes.iter_mut() {
            shape.transform(translate, scale);
        }
    }

    /// Write as Embedded Postscript (EPS)
    pub fn write_eps(&self, wr: &mut Write, min_width: f32, min_height: f32) -> io::Result<()> {
        let bounds = self.get_bounds();

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
%%HiResBoundingBox: {} {} {} {}
%%LanguageLevel: 2
%%Pages: 1
%%Page: 1 1
"#,
                      (bounds.min_x() - border_percent * width) as isize,
                      (bounds.min_y() - border_percent * height) as isize,
                      (bounds.max_x() + border_percent * width) as isize,
                      (bounds.max_y() + border_percent * height) as isize,

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

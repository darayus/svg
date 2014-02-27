// The MIT License (MIT)
//
// Copyright (c) 2014 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#[crate_id = "svg#0.0.1"];
#[desc = "SVG generation in Rust"];
#[license = "MIT"];
#[crate_type = "dylib"];
#[crate_type = "rlib"];
// #[warn(missing_doc)];
#[allow(dead_code)];

extern crate collections;

use std::io::{Writer, IoResult};
use std::fmt::Show;
use collections::HashMap;

pub use shapes::{Circle, Rect, RoundedRect, Ellipse, Line, PolyLine, Polygon};
pub use transform::Transform;

mod shapes;
mod transform;

static DOC_TYPE: &'static str = "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \
\"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
static XMLNS: &'static str = "version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" \
xmlns:xlink=\"http://www.w3.org/1999/xlink\">\n";
static STANDALONE_YES: &'static str = "<?xml version=\"1.0\" standalone=\"yes\"?>\n";
static STANDALONE_NO: &'static str = "<?xml version=\"1.0\" standalone=\"no\"?>\n";

trait SVGEntity {
    fn gen_output(&self) -> ~str;
}

pub fn rgb(red: u8,
           green: u8,
           blue: u8) -> ~str {
    format!("rgb({}, {}, {})", red, green, blue)
}

pub fn rgba(red: u8,
            green: u8,
            blue: u8,
            alpha: f32) -> ~str {
    format!("rgba({}, {}, {}, {})", red, green, blue, alpha)
}

struct Head {
    standalone: bool,
    width: i32,
    height: i32,
    view_box: Option<(i32, i32, i32, i32)>,
    desc: Option<~str>,
    title: Option<~str>
}

impl Head {
    pub fn new(width: i32, height: i32) -> Head {
        Head {
            standalone: false,
            width: width,
            height: height,
            view_box: None,
            desc: None,
            title: None
        }
    }
}

pub struct SVG<'a> {
    priv head: Head,
    priv content: ~str
}

fn make_attribs(attribs: &str) -> HashMap<~str, ~str>{
    let mut h = HashMap::new();
    for s in attribs.split(' ') {
        let t: ~[&str] = s.split('=').collect();
        h.insert(t[0].to_owned(), t[1].to_owned());
    }
    h
}

impl<'a> SVG<'a> {
    pub fn new(width: i32, height: i32) -> SVG {
        SVG {
            head: Head::new(width, height),
            content: ~""
        }
    }

    pub fn standalone(&mut self, standalone: bool) {
        self.head.standalone = standalone;
    }

    pub fn view_box(&mut self,
                    orig_x: i32,
                    orig_y: i32,
                    width: i32,
                    height: i32) {
        self.head.view_box = Some((orig_x, orig_y, width, height));
    }

    pub fn set_desc(&mut self, text: &str) {
        self.head.desc = Some(text.to_owned())
    }

    pub fn set_title(&mut self, text: &str) {
        self.head.title = Some(text.to_owned())
    }

    pub fn add<T: SVGEntity>(&mut self, new_entity: &T) {
        self.content.push_str(new_entity.gen_output());
    }

    pub fn circle(&mut self,
                  x: i32,
                  y: i32,
                  radius: u32,
                  attribs: &str) {
        self.content.push_str(Circle {
                x: x,
                y: y,
                radius: radius,
                attribs: make_attribs(attribs),
                transform: None
            }.gen_output())
    }

    pub fn rect(&mut self,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
                attribs: &str) {
        self.content.push_str(Rect {
                x: x,
                y: y,
                width: width,
                height: height,
                attribs: make_attribs(attribs),
                transform: None
            }.gen_output())
    }

    pub fn rounded_rect(&mut self,
                        x: i32,
                        y: i32,
                        width: i32,
                        height: i32,
                        x_round: u32,
                        y_round: u32,
                        attribs: &str) {
        self.content.push_str(RoundedRect {
                x: x,
                y: y,
                width: width,
                height: height,
                x_round: x_round,
                y_round: y_round,
                attribs: make_attribs(attribs),
                transform: None
            }.gen_output())
    }

    pub fn ellipse(&mut self,
                   x: i32,
                   y: i32,
                   x_radius: u32,
                   y_radius: u32,
                   attribs: &str) {
        self.content.push_str(Ellipse {
                x: x,
                y: y,
                x_radius: x_radius,
                y_radius: y_radius,
                attribs: make_attribs(attribs),
                transform: None
            }.gen_output())
    }

    pub fn line(&mut self,
                x1: i32,
                y1: i32,
                x2: i32,
                y2: i32,
                attribs: &str) {
        self.content.push_str(Line {
                x1: x1,
                y1: y1,
                x2: x2,
                y2: y2,
                attribs: make_attribs(attribs),
                transform: None
            }.gen_output())
    }

    pub fn polyline<T: Num + Show + Clone>(&mut self,
                                           points: &[(T, T)],
                                           attribs: &str) {
        self.content.push_str(PolyLine {
                points: points.to_owned(),
                attribs: make_attribs(attribs),
                transform: None
            }.gen_output())
    }

    pub fn polygon<T: Num + Show + Clone>(&mut self,
                                          points: &[(T, T)],
                                          attribs: &str) {
        self.content.push_str(Polygon {
                points: points.to_owned(),
                attribs: make_attribs(attribs),
                transform: None
            }.gen_output())
    }

    pub fn g_begin(&mut self,
                   id: Option<~str>,
                   transform: Option<&Transform>,
                   attribs: Option<&HashMap<~str, ~str>>) {
        self.content.push_str("<g ");
        match id {
            Some(i) => self.content.push_str(format!("id=\"{}\" ", i)),
            None    => {/* nothing to do */}
        }
        match transform {
            Some(t) => self.content.push_str(format!("{} ", t.get())),
            None    => {/* nothing to do */}
        }
        match attribs {
            Some(a) => {
                for (at, value) in a.iter() {
                    self.content.push_str(format!("{}=\"{}\" ", *at, *value))
                }
            },
            None    => {/* nothing to do */}
        }
        self.content.push_str(">\n");
    }

    pub fn g_id(&mut self, id: &str) {
        self.g_begin(Some(id.to_owned()), None, None)
    }

    pub fn g_transform(&mut self, transform: &Transform) {
        self.g_begin(None, Some(transform), None)
    }

    pub fn g_attribs(&mut self, attribs: &HashMap<~str, ~str>) {
        self.g_begin(None, None, Some(attribs))
    }

    pub fn g_translate(&mut self, x: i32, y: i32) {
        let mut t = Transform::new();
        t.translate(x, y);
        self.g_begin(None, Some(&t), None)
    }

    pub fn g_rotate(&mut self, angle: i32) {
        let mut t = Transform::new();
        t.rotate(angle);
        self.g_begin(None, Some(&t), None)
    }

    pub fn g_scale(&mut self, x_scale: i32, y_scale: i32) {
        let mut t = Transform::new();
        t.scale(x_scale, y_scale);
        self.g_begin(None, Some(&t), None)
    }

    // FIXME: test if a skew of 0 for y or x don't break
    pub fn g_skew(&mut self, x_factor: i32, y_factor: i32) {
        let mut t = Transform::new();
        t.skew_x(x_factor);
        t.skew_y(y_factor);
        self.g_begin(None, Some(&t), None)
    }

    pub fn g_end(&mut self) {
        self.content.push_str("</g>\n");
    }

    pub fn finalize(&mut self, output: &'a mut Writer) -> IoResult<()>{
        let mut o = ~"";
        // Head
        match self.head.standalone {
            true    => o.push_str(STANDALONE_YES),
            false   => o.push_str(STANDALONE_NO)
        };
        o.push_str(DOC_TYPE);
        o.push_str(format!("<svg width=\"{}cm\" height=\"{}cm\" ",
                           self.head.width, self.head.height));
        match self.head.view_box {
            Some((x, y, width, height)) => {
                o.push_str(format!("viewBox=\"{} {} {} {}\" ", x, y, width, height))
            },
            None                        => {/* nothing to do */}
        }
        o.push_str(XMLNS);
        match self.head.title {
            Some(ref t) => o.push_str(format!("<title>{}</title>\n", *t)),
            None    => {/* nothing to do */}
        }
        match self.head.desc {
            Some(ref d) => o.push_str(format!("<desc>{}</desc>\n", *d)),
            None    => {/* nothing to do */}
        }
        // Body
        o.push_str(self.content);
        // Close
        o.push_str(&"</svg>\n");
        output.write_str(o)
    }
}

use lib_plotings::svg::svg_circle;
use nannou::prelude::{pt2, Point2};
use nannou::Draw;
use svg::node::element::Ellipse;

use crate::massive::Massive;
use crate::params::Params;

pub struct FixedBody {
    mass: f32,
    radius: f32,
    xy: Point2,
}

impl FixedBody {
    pub fn draw(&self, draw: &Draw, params: &Params) {
        draw.ellipse()
            .xy(self.xy)
            .radius(self.radius)
            .no_fill()
            .stroke_weight(8.0);
    }

    pub fn svg(&self, params: &Params) -> Ellipse {
        svg_circle(self.xy, self.radius)
            .set("fill", "blue")
            .set("stroke", "lightblue")
    }
}

impl Massive for &FixedBody {
    fn mass(&self) -> f32 {
        self.mass
    }

    fn xy(&self) -> Point2 {
        self.xy
    }
}

impl Default for FixedBody {
    fn default() -> Self {
        Self {
            mass: 1000000.0,
            radius: 30.0,
            xy: pt2(0.0, 0.0),
        }
    }
}

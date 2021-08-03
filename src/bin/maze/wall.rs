use nannou::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use svg::node::element::Line;

use crate::params::MazeParams;

pub struct Wall {
    xy: Point2,
    kind: WallKind,
}

impl Wall {
    pub fn new(xy: Point2, rng: &mut impl Rng) -> Self {
        let kind = rng.gen();

        Self { xy, kind }
    }

    pub fn xy(&self) -> &Point2 {
        &self.xy
    }

    pub fn kind(&self) -> WallKind {
        self.kind
    }

    pub fn draw(&self, draw: &Draw, params: &MazeParams) {
        let gcw = params.grid_cell_width as f32;
        let gch = params.grid_cell_height as f32;

        let x = self.xy().x * gcw;
        let y = self.xy().y * gch;
        let start = match self.kind() {
            WallKind::A => pt2(0.0, 0.0),
            WallKind::B => pt2(0.0, -gch),
        };
        let end = match self.kind() {
            WallKind::A => pt2(gcw, -gch),
            WallKind::B => pt2(gcw, 0.0),
        };

        draw.line()
            .x_y(x, y)
            .points(start, end)
            .weight(3.0)
            .end_cap_round();
    }

    pub fn svg(&self, params: &MazeParams) -> svg::node::element::Line {
        let gcw = params.grid_cell_width as f32;
        let gch = params.grid_cell_height as f32;

        let x = self.xy().x * gcw;
        let y = self.xy().y * gch;
        let start = match self.kind() {
            WallKind::A => pt2(0.0, 0.0),
            WallKind::B => pt2(0.0, -gch),
        };
        let end = match self.kind() {
            WallKind::A => pt2(gcw, -gch),
            WallKind::B => pt2(gcw, 0.0),
        };

        Line::new()
            .set("x1", x + start.x)
            .set("y1", y + start.y + gch)
            .set("x2", x + end.x)
            .set("y2", y + end.y + gch)
    }
}

#[derive(Clone, Copy)]
pub enum WallKind {
    A,
    B,
}

impl Distribution<WallKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WallKind {
        if rng.gen_bool(0.5) {
            WallKind::A
        } else {
            WallKind::B
        }
    }
}

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
    is_hovered: bool,
}

impl Wall {
    pub fn new(xy: Point2, rng: &mut impl Rng) -> Self {
        let kind = rng.gen();

        Self {
            xy,
            kind,
            is_hovered: false,
        }
    }

    pub fn xy(&self) -> &Point2 {
        &self.xy
    }

    pub fn kind(&self) -> WallKind {
        self.kind
    }

    pub fn flip(&mut self) {
        self.kind = match self.kind {
            WallKind::A => WallKind::B,
            WallKind::B => WallKind::A,
        };
    }

    pub fn set_is_hovered(&mut self, is_hovered: bool) {
        self.is_hovered = is_hovered;
    }

    /// Get a reference to the wall's is hovered.
    pub fn is_hovered(&self) -> bool {
        self.is_hovered
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

        if self.is_hovered {
            let (x, y) = (x + gcw / 2.0, y - gch / 2.0);

            draw.rect()
                .w_h(gcw, gch)
                .x_y(x, y)
                .no_fill()
                .stroke(RED)
                .stroke_weight(1.0);
        }
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

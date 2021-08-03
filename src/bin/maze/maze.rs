use crate::params::MazeParams;
use crate::wall::{Wall, WallKind};
use nannou::prelude::*;
use rand::Rng;
use svg::node::element::Line;

pub struct Maze {
    walls: Vec<Wall>,
}

impl Maze {
    pub fn new(params: &MazeParams, rng: &mut impl Rng) -> Self {
        let walls = (0..params.rows)
            .flat_map(|y| (0..params.columns).map(move |x| pt2(x as f32, y as f32)))
            .map(|xy| Wall::new(xy, rng))
            .collect();

        Self { walls }
    }

    pub fn update(&mut self, _params: &MazeParams, _rng: &mut impl Rng) {
        // do nothing
    }

    pub fn draw(&self, draw: &Draw, params: &MazeParams) {
        let maze_width = params.width();
        let maze_height = params.height();
        let centered_origin = vec3(
            maze_width * -0.5,
            maze_height * -0.5 + params.grid_cell_height as f32,
            0.0,
        );
        let draw = draw.translate(centered_origin);

        self.walls.iter().for_each(|wall| wall.draw(&draw, &params));
    }

    pub fn svg(&self, params: &MazeParams) -> svg::node::element::Group {
        let mut group = svg::node::element::Group::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "0.3mm");

        for wall in self.walls().iter() {
            let line = wall.svg(params);

            group = group.add(line);
        }

        group
    }

    pub fn walls(&self) -> &[Wall] {
        &self.walls
    }
}

impl Default for Maze {
    fn default() -> Self {
        Self { walls: Vec::new() }
    }
}

use crate::params::MazeParams;
use crate::wall::Wall;
use lib_plotings::MouseButtonState;
use log::{info, trace};
use nannou::prelude::*;
use rand::Rng;

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

    pub fn update(
        &mut self,
        params: &MazeParams,
        mouse_tile_position: Option<Point2>,
        mouse_button_state: MouseButtonState,
    ) {
        if let Some(xy) = mouse_tile_position {
            self.walls
                .iter_mut()
                .for_each(|wall| wall.set_is_hovered(wall.xy() == &xy));

            if mouse_button_state.is_just_pressed() {
                trace!("Left mouse button click at {:?}", mouse_tile_position);
                self.flip_wall_at(&xy);
            }
        }
    }

    pub fn new_walls_from_rng(&mut self, params: &MazeParams, rng: &mut impl Rng) {
        self.walls = (0..params.rows)
            .flat_map(|y| (0..params.columns).map(move |x| pt2(x as f32, y as f32)))
            .map(|xy| Wall::new(xy, rng))
            .collect();
    }

    pub fn flip_wall_at(&mut self, xy: &Point2) {
        if let Some(wall) = self.walls.iter_mut().filter(|wall| wall.xy() == xy).next() {
            wall.flip();
        }
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

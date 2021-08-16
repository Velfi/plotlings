use crate::massive::Massive;
use crate::params::Params;
use lib_plotings::svg::svg_circle;
use log::debug;
use nannou::prelude::*;
use once_cell::sync::Lazy;
use rand::Rng;
use std::collections::VecDeque;
use svg::node::element::Group;

pub const GRAVITATIONAL_CONSTANT: Lazy<f32> = Lazy::new(|| 6.67428 * (10.0.powi(-11)));
pub const GRAVITY_MULTIPLIER: f32 = 1000.0;

#[derive(Debug, Clone)]
pub struct Orbiter {
    mass: f32,
    radius: f32,
    velocity: Vec2,
    current_xy: Point2,
    forces_to_be_applied: Vec<Vec2>,
    trail: VecDeque<Point2>,
    start_xy: Point2,
}

impl Orbiter {
    pub fn new(mass: f32, radius: f32, velocity: Vec2, xy: Point2, max_trail_dots: usize) -> Self {
        Self {
            current_xy: xy,
            start_xy: xy,
            velocity,
            mass,
            radius,
            forces_to_be_applied: Default::default(),
            trail: VecDeque::with_capacity(max_trail_dots),
        }
    }

    pub fn new_from_params(params: &Params, rng: &mut impl Rng) -> Self {
        let xy = pt2(params.width / 2.0, params.height / 2.0);
        let mass = rng.gen_range(params.mass_range.clone());
        let radius = rng.gen_range(params.radius_range.clone());
        let velocity = vec2(-0.2, -1.0);

        Self::new(mass, radius, velocity, xy, params.max_trail_dots)
    }

    pub fn queue_force_update(&mut self, other: impl Massive) {
        // Newton's law of universal gravitation
        // https://en.wikipedia.org/wiki/Newton's_law_of_universal_gravitation
        let force_of_gravity = (*GRAVITATIONAL_CONSTANT * self.mass * other.mass())
            / self.current_xy.distance_squared(other.xy())
            * GRAVITY_MULTIPLIER;

        let direction_of_force = other.xy() - self.current_xy;
        let force_vector =
            direction_of_force.try_normalize().unwrap_or_default() * force_of_gravity;

        debug!("Applying force of {:?} to orbiter", &force_vector);

        self.forces_to_be_applied.push(force_vector);
    }

    pub fn update(&mut self, params: &Params, tick_has_occurred: bool) {
        for force in self.forces_to_be_applied.drain(..) {
            self.velocity += force;
        }

        self.current_xy += self.velocity;

        if tick_has_occurred {
            self.trail.push_back(self.current_xy);

            if self.trail.len() > params.max_trail_dots {
                let _ = self.trail.pop_front();
            }
        }
    }

    pub fn draw(&self, draw: &Draw, _params: &Params) {
        for trailer_xy in self.trail.iter() {
            draw.ellipse()
                .xy(trailer_xy.clone())
                .radius(2.0)
                .no_fill()
                .stroke_weight(1.0);
        }

        draw.ellipse()
            .xy(self.current_xy)
            .radius(self.radius)
            .no_fill()
            .stroke_weight(3.0);
    }

    pub fn svg(&self, _params: &Params) -> Group {
        let mut trailers = Group::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", "1");

        for trail_xy in self.trail.iter() {
            let dot = svg_circle(*trail_xy, 4.0);

            trailers = trailers.add(dot);
        }

        Group::new()
            // starting position
            .add(
                svg_circle(self.start_xy, self.radius)
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", "5"),
            )
            // ending position
            .add(
                svg_circle(self.current_xy, self.radius)
                    .set("fill", "black")
                    .set("stroke", "none"),
            )
            // trailers
            .add(trailers)
    }
}

impl Massive for &Orbiter {
    fn mass(&self) -> f32 {
        self.mass
    }

    fn xy(&self) -> Point2 {
        self.current_xy
    }
}

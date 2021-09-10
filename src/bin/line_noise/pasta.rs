use super::State;
use nannou::prelude::*;
use noise::{NoiseFn, SuperSimplex};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use svg::node::element::Polyline;

pub struct Line {
    start: Vec2,
    end: Vec2,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }

    pub fn draw(&self, draw: &Draw, stroke_weight: f32) {
        draw.polyline()
            .weight(stroke_weight)
            .caps_round()
            .points(vec![self.start.clone(), self.end.clone()]);
    }

    pub fn as_svg(&self) -> Polyline {
        let points = format!(
            "{:.2},{:.2} {:.2},{:.2}",
            self.start.x, self.start.y, self.end.x, self.end.y
        );

        Polyline::new().set("points", points)
    }
}

pub type Lines = Vec<Line>;

pub fn new_lines_from_noise(params: &State) -> Lines {
    let noise_fn = SuperSimplex::new();
    let mut rng: StdRng = SeedableRng::seed_from_u64(params.noise_seed);
    let horizontal_line_spacing = params.width as f64 / params.line_count as f64;
    (0..=params.line_count)
        .flat_map(|index| {
            let mut subline = Vec::new();
            let mut y = 0.0;
            let x = index as f64 * horizontal_line_spacing;
            let mut is_drawing = false;
            let mut line_start = None;
            let y_jitter = rng.gen_range(-params.vertical_jitter..params.vertical_jitter);

            loop {
                if y > params.height {
                    break;
                }

                let dice_roll: f64 = rng.gen();
                let chance_of_state_change = {
                    let (x, y) = (x * params.noise_scale, y as f64 * params.noise_scale);
                    noise_fn.get([params.noise_seed as f64, x, y])
                };

                if is_drawing {
                    // roll dice to see if we should stop drawing
                    if dice_roll > chance_of_state_change {
                        is_drawing = false;
                        let start = line_start.take().expect("No line start was set?");
                        let end = Vec2::new(x as f32, y + y_jitter);

                        subline.push(Line::new(start, end))
                    }
                } else {
                    // roll dice to see if we should start drawing
                    if dice_roll < chance_of_state_change {
                        is_drawing = true;
                        line_start = Some(Vec2::new(x as f32, y + y_jitter));
                    }
                }

                y += params.min_line_length;
            }

            subline.into_iter()
        })
        .collect()
}

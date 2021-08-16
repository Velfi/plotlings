use std::ops::Range;

pub struct Params {
    pub width: f32,
    pub height: f32,
    pub radius_range: Range<f32>,
    pub mass_range: Range<f32>,
    pub steps_to_simulate: usize,
    pub rng_seed: u64,
    pub orbiter_count: u8,
    pub max_trail_dots: usize,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 800.0,
            steps_to_simulate: 100,
            rng_seed: 0,
            mass_range: 100.0..1000.0,
            radius_range: 5.0..20.0,
            orbiter_count: 1,
            max_trail_dots: 100,
        }
    }
}

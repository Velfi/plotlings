pub struct CircleParams {
    pub min_radius: f32,
    pub max_radius: f32,
    pub start_position_variance: f32,
}

impl Default for CircleParams {
    fn default() -> Self {
        Self {
            min_radius: 9.0,
            max_radius: 10.0,
            start_position_variance: 1.0,
        }
    }
}

pub struct PackerParams {
    pub circle_count: usize,
    pub width: f32,
    pub height: f32,
}

impl Default for PackerParams {
    fn default() -> Self {
        Self {
            circle_count: 1000,
            width: 1000.0,
            height: 1000.0,
        }
    }
}

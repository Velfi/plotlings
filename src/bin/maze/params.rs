pub struct MazeParams {
    pub grid_cell_width: usize,
    pub grid_cell_height: usize,
    pub columns: usize,
    pub rows: usize,
    pub rng_seed: u64,
}

impl MazeParams {
    pub fn width(&self) -> f32 {
        (self.grid_cell_width * self.columns) as f32
    }

    pub fn height(&self) -> f32 {
        (self.grid_cell_height * self.rows) as f32
    }
}

impl Default for MazeParams {
    fn default() -> Self {
        Self {
            grid_cell_width: 20,
            grid_cell_height: 20,
            columns: 55,
            rows: 40,
            rng_seed: 0,
        }
    }
}

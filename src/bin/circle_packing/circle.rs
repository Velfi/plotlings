use log::trace;
use nannou::prelude::*;

#[derive(Debug, Default)]
pub struct Circle {
    pub xy: Vec2,
    pub radius: f32,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Circle {
    pub fn new(xy: Vec2, radius: f32) -> Self {
        assert!(
            radius != 0.0,
            "I'm pretty sure I don't want 0-radius circles"
        );

        Self {
            xy,
            radius,
            ..Default::default()
        }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        let combined_radius = self.radius + other.radius;
        // `distance_squared` is less expensive than `distance`
        self.xy.distance_squared(other.xy) < combined_radius.powi(2)
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update(&mut self) {
        trace!("self.acceleration={}", &self.acceleration);
        self.velocity += self.acceleration;
        trace!("self.velocity={}", &self.velocity);
        self.xy += self.velocity;
        trace!("self.xy={}", &self.xy);
        self.acceleration *= 0.0;
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.xy)
            .radius(self.radius)
            // 0.3mm Marker
            // .stroke_weight(1.0)
            // Molotow Marker
            .stroke_weight(12.0)
            .no_fill();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlapping() {
        let a = Circle::new(Vec2::new(0.0, 0.0), 1.0);
        let b = Circle::new(Vec2::new(0.0, 0.0), 1.0);

        assert!(a.overlaps(&b));

        let a = Circle::new(Vec2::new(10.0, 0.0), 1.0);
        let b = Circle::new(Vec2::new(10.0, 10.0), 10.0);

        assert!(a.overlaps(&b));
    }

    #[test]
    fn test_not_overlapping() {
        let a = Circle::new(Vec2::new(0.0, 0.0), 1.0);
        let b = Circle::new(Vec2::new(2.0, 2.0), 1.0);

        assert!(!a.overlaps(&b));
    }
}

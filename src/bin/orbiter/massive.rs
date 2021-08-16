use nannou::prelude::Point2;

pub trait Massive {
    fn mass(&self) -> f32;
    fn xy(&self) -> Point2;
}

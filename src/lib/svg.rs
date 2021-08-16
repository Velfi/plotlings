use nannou::prelude::*;
use svg::node::element::Ellipse;

pub fn svg_ellipse(xy: Point2, radius_x: f32, radius_y: f32) -> Ellipse {
    Ellipse::new()
        .set("cx", xy.x)
        .set("cy", xy.y)
        .set("rx", radius_x)
        .set("ry", radius_y)
}

pub fn svg_circle(xy: Point2, radius: f32) -> Ellipse {
    Ellipse::new()
        .set("cx", xy.x)
        .set("cy", xy.y)
        .set("rx", radius)
        .set("ry", radius)
}

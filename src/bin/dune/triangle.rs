use nannou::prelude::*;
use noise::{NoiseFn, SuperSimplex};
use svg::node::element::Polyline;

pub struct Triangle {
    points: [Point2; 3],
    // /// The position of a triangle
    // xy: Point2,
    /*
    |\                   /\                   /|
    | \                 /  \                 / |
    |  \      ->       /    \     ->        /  |
    |___\             /______\             /___|

    right-angle,    equilateral     right-angle,
    left aligned                  right aligned
    0.0      ->        skew       ->        1.0
    */
    // /// Controls whether a triangle leans to the left or to the right. See the definition of this struct for a diagram
    // skew: f32,
    // width: f32,
    // height: f32,
}

impl Triangle {
    pub fn new(xy: Vec2, skew: f32, width: f32, height: f32) -> Self {
        let points = generate_triangle_points_from_attributes(xy, skew, width, height);

        Self { points }
    }

    pub fn draw(&self, draw: &Draw) {
        let points = self.points.iter().cloned();

        draw.polyline()
            .weight(1.0)
            .caps_round()
            .points_closed(points);
    }

    pub fn bounding_rect(&self) -> nannou::geom::Rect {
        let (l, r, t, b) = self.points.iter().fold(
            (f32::MAX, f32::MIN, f32::MAX, f32::MIN),
            |(mut l, mut r, mut b, mut t), xy| {
                if xy.x < l {
                    l = xy.x;
                };
                if xy.x > r {
                    r = xy.x;
                };
                if xy.x < b {
                    b = xy.x;
                };
                if xy.y > t {
                    t = xy.y;
                };

                (l, r, b, t)
            },
        );

        let (a, b) = (vec2(l, t), vec2(r, b));

        nannou::geom::Rect::from_corners(a, b)
    }

    pub fn as_svg(&self) -> Polyline {
        let data: Vec<_> = self
            .points
            .iter()
            // have to loop around to the first point again
            // so we make a cycle iter and take 4
            .cycle()
            .take(4)
            .map(|p| format!("{:.2},{:.2}", p.x, p.y))
            .collect();

        Polyline::new().set("points", data.join(" "))
    }
}

fn generate_triangle_points_from_attributes(
    xy: Vec2,
    skew: f32,
    width: f32,
    height: f32,
) -> [Vec2; 3] {
    let (origin_x, origin_y) = (xy.x - width / 2.0, xy.y - height / 2.0);

    [
        Vec2::new(origin_x, origin_y),
        Vec2::new(origin_x + (width * skew), origin_y + height),
        Vec2::new(origin_x + width, origin_y),
    ]
}

pub type Triangles = Vec<Triangle>;

pub fn bounding_rect_from_triangles(triangles: &[Triangle]) -> nannou::geom::Rect {
    let (l, r, t, b) = triangles
        .iter()
        .map(|tri| tri.bounding_rect())
        //       Left      Right     Bottom    Top
        .fold(
            (f32::MAX, f32::MIN, f32::MAX, f32::MIN),
            |(mut l, mut r, mut b, mut t), tri_rect| {
                let (tri_l, tri_r, tri_b, tri_t) = tri_rect.l_r_b_t();

                if tri_l < l {
                    l = tri_l;
                };
                if tri_r > r {
                    r = tri_r;
                };
                if tri_b < b {
                    b = tri_b;
                };
                if tri_t > t {
                    t = tri_t;
                };

                (l, r, b, t)
            },
        );
    let (a, b) = (vec2(l, t), vec2(r, b));

    nannou::geom::Rect::from_corners(a, b)
}

pub fn new_triangles_from_noise(params: &TriangleParams) -> Triangles {
    let noise_fn = SuperSimplex::new();
    let halfway_point = (params.count / 2) as f32;
    (0..=params.count)
        .map(|index| {
            let index = index as f32;
            let count = params.count as f32;
            let t = if index < halfway_point {
                (index / count) * 2.0
            } else {
                ((count - index) / count) * 2.0
            };
            let x = (noise_fn.get([params.noise_seed, index as f64 * params.noise_scale]) * 200.0)
                as f32;
            let xy = pt2(x, index * -params.vertical_spacing);
            let height = map_range(t, 0.0, 1.0, params.min_height, params.max_height);
            let width = height * params.wh_ratio;

            Triangle::new(xy, params.skew, width, height)
        })
        .collect()
}

// pub fn new_triangles_from_parabola(
//     params: &TriangleParams,
//     origin: Vec2,
//     a: f32,
//     b: f32,
//     c: f32,
// ) -> Triangles {
//     if params.count == 0 {
//         Vec::new()
//     } else {
//         let (starting_x, ending_x) = quadratic_formula(a, b, c);
//         (0..=params.count)
//             .map(|index| {
//                 let t = index as f32 / params.count as f32;
//                 let par_x = starting_x.interpolate(ending_x, t);
//                 let size = a * par_x.exp2() + b * par_x + c;
//
//                 let xy = pt2(0.0, index as f32 * -params.vertical_spacing);
//
//                 Triangle::new(
//                     xy + origin,
//                     params.skew,
//                     size * params.scale * params.wh_ratio,
//                     size * params.scale,
//                 )
//             })
//             .collect()
//     }
// }
//
// fn quadratic_formula(a: f32, b: f32, c: f32) -> (f32, f32) {
//     assert_ne!(a, 0.0, "not a quadratic equasion");
//
//     let delta = (b.powi(2) - (4.0 * a * c)).sqrt();
//     let x1 = (-b + delta) / (2.0 * a);
//     let x2 = (-b - delta) / (2.0 * a);
//
//     (x1, x2)
// }

#[derive(Debug, PartialEq, Clone)]
pub struct TriangleParams {
    pub count: u32,
    pub wh_ratio: f32,
    pub skew: f32,
    pub vertical_spacing: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub noise_seed: f64,
    pub noise_scale: f64,
}

impl Default for TriangleParams {
    fn default() -> Self {
        Self {
            count: 120,
            skew: 0.5,
            vertical_spacing: 5.0,
            max_height: 200.0,
            min_height: 10.0,
            wh_ratio: 1.61803398875,
            noise_seed: 0.0,
            noise_scale: 0.02,
        }
    }
}

// <polygon points="0,100 50,25 50,75 100,0" />

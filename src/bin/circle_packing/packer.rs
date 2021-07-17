use std::{
    cell::RefCell,
    ops::{AddAssign, SubAssign},
};

use crate::{circle::Circle, Model};
use log::trace;
use nannou::prelude::*;
use rand::Rng;

#[derive(Debug, Default)]
pub struct Packer {
    pub circles: Vec<RefCell<Circle>>,
    max_speed: f32,
    max_force: f32,
}

impl Packer {
    pub fn new(model: &Model) -> Self {
        let circles = (0..=model.packer_params.circle_count)
            .map(|_| {
                let mut rng = model.rng.borrow_mut();
                let x = rng.gen_range(
                    -model.circle_params.start_position_variance
                        ..model.circle_params.start_position_variance,
                );
                let y = rng.gen_range(
                    -model.circle_params.start_position_variance
                        ..model.circle_params.start_position_variance,
                );
                let xy = Vec2::new(x, y);
                let radius =
                    rng.gen_range(model.circle_params.min_radius..model.circle_params.max_radius);

                RefCell::new(Circle::new(xy, radius))
            })
            .collect();

        Self {
            circles,
            max_force: 10.0,
            max_speed: 10.0,
        }
    }

    pub fn add_circle(&mut self, circle: Circle) {
        self.circles.push(RefCell::new(circle))
    }

    pub fn is_empty(&self) -> bool {
        self.circles.is_empty()
    }

    pub fn len(&self) -> usize {
        self.circles.len()
    }

    pub fn draw(&self, draw: &Draw) {
        self.circles
            .iter()
            .for_each(|circle| circle.borrow().draw(draw))
    }

    pub fn update(&mut self, width: f32, height: f32) {
        trace!("packer.update() called");
        let mut separate_forces: Vec<RefCell<Vec2>> = (0..self.len())
            .into_iter()
            .map(|_| Default::default())
            .map(RefCell::new)
            .collect();
        let mut near_circles: Vec<usize> = (0..self.len())
            .into_iter()
            .map(|_| Default::default())
            .collect();

        let boundary = Rect::from_w_h(width, height);

        for circle_index in 0..self.circles.len() {
            check_borders(circle_index, &self.circles, &boundary);
            check_circle_position(circle_index, &self.circles);
            apply_separation_forces_to_circle(
                circle_index,
                &self.circles,
                &mut separate_forces,
                &mut near_circles,
                self.max_force,
                self.max_speed,
            )
        }
    }
}

// TODO should this call circle.update()? That feels wrong. Disabling for now
pub fn check_borders(index: usize, circles: &[RefCell<Circle>], boundary: &Rect) {
    let mut circle = circles.get(index).unwrap().borrow_mut();
    let x = circle.xy.x;
    let y = circle.xy.y;
    let half_r = circle.radius / 2.0;

    if x - half_r < boundary.left() || x + half_r > boundary.right() {
        trace!(
            "circle #{} was outside x bounds, reversing velocity x",
            &index
        );
        circle.velocity.x *= -1.0;
        // circle.update();
    }

    if y - half_r < boundary.bottom() || y + half_r > boundary.top() {
        trace!(
            "circle #{} was outside y bounds, reversing velocity y",
            &index
        );
        circle.velocity.y *= -1.0;
        // circle.update();
    }
}

pub fn check_circle_position(index: usize, circles: &[RefCell<Circle>]) {
    let mut circle_i = circles.get(index).unwrap().borrow_mut();
    let mut circle_has_a_neighbor = false;

    for j in (index + 1)..circles.len() {
        let circle_j = circles.get(j).unwrap().borrow_mut();

        if circle_i.overlaps(&circle_j) {
            circle_has_a_neighbor = true;
            break;
        }
    }

    // Zero velocity if no neighbours
    if !circle_has_a_neighbor {
        trace!("circle {} has no neighbors, zeroing velocity", &index);
        circle_i.velocity.x = 0.0;
        circle_i.velocity.y = 0.0;
    }
}

pub fn apply_separation_forces_to_circle(
    index: usize,
    circles: &[RefCell<Circle>],
    separate_forces: &mut [RefCell<Vec2>],
    near_circles: &mut [usize],
    max_force: f32,
    max_speed: f32,
) {
    let mut separate_forces_i = separate_forces.get(index).unwrap().borrow_mut();
    let mut circle = circles.get(index).unwrap().borrow_mut();

    for j in (index + 1)..circles.len() {
        let j = if j == circles.len() { 0 } else { j };
        assert_ne!(index, j);

        let mut separate_forces_j = separate_forces.get(j).unwrap().borrow_mut();
        let circle_j = circles.get(j).unwrap().borrow_mut();

        let force_ij = get_separation_force(&circle, &circle_j);

        if force_ij.distance(Vec2::default()) > 0.0 {
            trace!(
                "separation force between {} and {} == {}",
                index,
                j,
                force_ij
            );
            separate_forces_i.add_assign(force_ij);
            separate_forces_j.sub_assign(force_ij);
            near_circles[index] += 1;
            near_circles[j] += 1;
        }
    }

    if near_circles[index] > 0 {
        *separate_forces_i /= near_circles[index] as f32;
    }

    if separate_forces_i.distance(Vec2::default()) > 0.0 {
        *separate_forces_i = set_mag(*separate_forces_i, max_speed);
        *separate_forces_i -= circle.velocity;
        *separate_forces_i = limit(*separate_forces_i, max_force);
    }

    trace!("applying {:?} force to circle {}", separate_forces_i, index);

    circle.apply_force(*separate_forces_i);
    circle.update();
}

pub fn get_separation_force(n1: &Circle, n2: &Circle) -> Vec2 {
    let mut steer = Vec2::new(0.0, 0.0);
    let d = n1.xy.distance(n2.xy);

    if (d > 0.0) && (d < n1.radius / 2.0 + n2.radius / 2.0) {
        let diff = (n1.xy - n2.xy).normalize() / d;

        steer += diff;
    } else {
        trace!("no separation force between circles {:?} and {:?}", n1, n2);
    }

    steer
}

fn set_mag(mut v: Vec2, mag: f32) -> Vec2 {
    v = v.normalize();
    v *= mag;

    v
}

fn limit(mut v: Vec2, limit: f32) -> Vec2 {
    if v.distance_squared(Vec2::default()) > limit.powi(2) {
        v = v.normalize();
        v *= limit;
    }
    v
}

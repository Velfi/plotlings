use crate::fixed_body::FixedBody;
use crate::{orbiter::Orbiter, params::Params};
use lib_plotings::interval::Interval;
use lib_plotings::MouseButtonState;
use log::trace;
use nannou::prelude::*;
use rand::Rng;
use svg::node::element::Group;

pub struct System {
    orbiters: Vec<Orbiter>,
    fixed_bodies: Vec<FixedBody>,
    orbital_ticker: Interval,
}

impl System {
    pub fn new_from_params(params: &Params, rng: &mut impl Rng) -> Self {
        let orbiters = (0..params.orbiter_count)
            .into_iter()
            .map(|_index| Orbiter::new_from_params(params, rng))
            .collect();

        let fixed_bodies = vec![FixedBody::default()];
        let orbital_ticker = Interval::new_from_seconds(1);

        Self {
            orbiters,
            fixed_bodies,
            orbital_ticker,
        }
    }

    pub fn update(
        &mut self,
        params: &Params,
        mouse_position: Option<Point2>,
        mouse_button_state: MouseButtonState,
    ) {
        let orbital_tick_occurred = self.orbital_ticker.tick();

        if let Some(_xy) = mouse_position {
            // mouse position stuff

            if mouse_button_state.is_just_pressed() {
                trace!("Left mouse button click at {:?}", mouse_position);
            }
        }
        let cloned_orbiters = self.orbiters.clone();

        // Queue up forces of gravity
        for (index, orbiter) in &mut self.orbiters.iter_mut().enumerate() {
            // force of fixed bodies
            for fb in self.fixed_bodies.iter() {
                orbiter.queue_force_update(fb);
            }

            // force of other orbiters
            for (clone_index, clone_orbiter) in cloned_orbiters.iter().enumerate() {
                if index == clone_index {
                    continue;
                }

                orbiter.queue_force_update(clone_orbiter);
            }
        }

        // Apply queued forces to each orbiter
        for orbiter in self.orbiters.iter_mut() {
            orbiter.update(params, orbital_tick_occurred);
        }
    }

    pub fn draw(&self, draw: &Draw, params: &Params) {
        self.fixed_bodies
            .iter()
            .for_each(|fixed_body| fixed_body.draw(draw, params));

        self.orbiters
            .iter()
            .for_each(|orbiter| orbiter.draw(draw, params));
    }

    pub fn svg(&self, params: &Params) -> Group {
        let mut group = Group::new();

        for orbiter in self.orbiters.iter() {
            let orbiter_group = orbiter.svg(params);

            group = group.add(orbiter_group);
        }

        group
    }

    /// Get a reference to the system's orbiters.
    pub fn orbiters(&self) -> &[Orbiter] {
        self.orbiters.as_slice()
    }
}

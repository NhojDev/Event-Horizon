use crate::physics::Body;
use ggez::glam::Vec2;

/// All tunable parameters for the simulation.
pub struct SimConfig {
    pub particle_count: usize,
    pub dt: f32,

    pub base_mass: f32,
    pub base_radius: f32,

    pub spawn_width: f32,
    pub spawn_height: f32,

    pub min_gravity: f32,
    pub max_gravity: f32,
}

impl SimConfig {
    pub fn default() -> Self {
        Self {
            particle_count: 500,
            dt: 0.5,

            base_mass: 10.0,
            base_radius: 3.0,

            spawn_width: 800.0,
            spawn_height: 600.0,

            min_gravity: 0.05,
            max_gravity: 0.5,
        }
    }

    /// Spawns a list of randomized bodies
    pub fn spawn_bodies(&self) -> Vec<Body> {
        let mut bodies = Vec::with_capacity(self.particle_count);

        for _ in 0..self.particle_count {
            let gravity =
                rand::random::<f32>() * (self.max_gravity - self.min_gravity) + self.min_gravity;

            let mass = self.base_mass * (gravity * 0.8 + 0.4);

            bodies.push(Body {
                pos: Vec2::new(
                    rand::random::<f32>() * self.spawn_width - (self.spawn_width / 2.0),
                    rand::random::<f32>() * self.spawn_height - (self.spawn_height / 2.0),
                ),
                vel: Vec2::ZERO,

                radius: self.base_radius,
                mass,
                gravity,
            });
        }

        bodies
    }
}
